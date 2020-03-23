use crate::intra::{autologin, check, client, format};
use crate::v1::data;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

#[get("/day")]
pub async fn day(req: HttpRequest, input: web::Json<data::PlanningDayInput>) -> impl Responder {
    let autologin = match autologin::get_from_header(&req) {
        Some(autologin) => autologin,
        _ => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("no autologin provided"),
            })
        }
    };

    match autologin::check(&autologin) {
        Some(result) => {
            if result == false {
                return HttpResponse::BadRequest().json(data::Default {
                    msg: String::from("bad autologin provided"),
                });
            }
        }
        None => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("failed to check autologin"),
            })
        }
    }

    let full_date = match check::yyyy_mm_dd(&input.date) {
        Some(full_date) => full_date,
        None => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("invalid date provided"),
            });
        }
    };

    match check::check::email(&input.email) {
        Some(res) => {
            if res == true {
                ()
            } else {
                return HttpResponse::BadRequest().json(data::Default {
                    msg: String::from("field `email` is invalid"),
                });
            }
        }
        None => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("field `email` failed to verify"),
            });
        }
    }

    let client = match client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let formatted_date = full_date.format("%Y-%m-%d").to_string();
    let path = format!(
        "/planning/load?format=json&start={}&end={}",
        formatted_date, formatted_date
    );
    let res = match client::get_path_auth(&client, &autologin, &path).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("client error"),
            })
        }
    };

    if res.status() != StatusCode::OK {
        return HttpResponse::InternalServerError().json(data::Default {
            msg: String::from("could not get custom_planning information"),
        });
    }

    let raw_body = match res.text().await {
        Ok(raw_body) => raw_body,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not get intra response"),
            })
        }
    };

    let mut list: Vec<data::PlanningDayResult> = Vec::new();

    // if json parsing fails, that means there are no events
    // json parsing fails because the intra returns an empty object
    // and we are expecting a vector
    let raw_json: Vec<Value> = match serde_json::from_str(&raw_body) {
        Ok(raw_json) => raw_json,
        Err(_) => return HttpResponse::Ok().json(list),
    };

    for event in &raw_json {
        // Get the semester of the event
        let semester_event = match event["semester"].as_u64() {
            Some(semester_event) => semester_event,
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `semester` does not exist"),
                })
            }
        };

        // Rules to save event:
        //
        // User is a privileged account (aer, ape, adm) -> input.current_semester == 42
        // Event does not fit in a semester (french, english, hub, etc) -> semester_event == 0
        // Event belongs to previous semester -> semester_event == (input.current_semester - 1)
        // Event belongs to current semester -> semester_event == input.current_semester
        let save_event = input.current_semester == 42
            || semester_event == 0
            || semester_event == (input.current_semester - 1)
            || semester_event == input.current_semester;

        if save_event == false {
            // Skip this event, move to the next one
            continue;
        }

        // Store fields in temporary values

        let is_custom: bool = false;

        let is_rdv: bool = match event["is_rdv"].as_str() {
            Some(is_rdv) => {
                if is_rdv == "1" {
                    true
                } else {
                    false
                }
            }
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `is_rdv` does not exist"),
                })
            }
        };

        let is_regular: bool = match is_rdv {
            true => false,
            false => true,
        };

        let year: u64 = match event["scolaryear"].as_str() {
            Some(year) => match year.parse() {
                Ok(year) => year,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `scolaryear` is not a number"),
                    })
                }
            },
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `scolaryear` does not exist"),
                })
            }
        };

        let code_module: String = match event["codemodule"].as_str() {
            Some(code_module) => String::from(code_module),
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `codemodule` does not exist"),
                })
            }
        };

        let code_instance: String = match event["codeinstance"].as_str() {
            Some(code_instance) => String::from(code_instance),
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `codeinstance` does not exist"),
                })
            }
        };

        let code_acti: String = match event["codeacti"].as_str() {
            Some(code_acti) => String::from(code_acti),
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `codeacti` does not exist"),
                })
            }
        };

        let code_event: String = match is_regular {
            true => match event["codeevent"].as_str() {
                Some(code_event) => String::from(code_event),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `codeevent` does not exist"),
                    })
                }
            },
            false => String::new(),
        };

        let title: String = match event["acti_title"].as_str() {
            Some(acti_title) => String::from(acti_title),
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `acti_title` does not exist"),
                })
            }
        };

        let module: String = match event["titlemodule"].as_str() {
            Some(titlemodule) => String::from(titlemodule),
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `titlemodule` does not exist"),
                })
            }
        };

        let room: String = match event["room"]["code"].as_str() {
            Some(room) => match format::room(room) {
                Some(room) => room,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("formatting value `room.code` failed"),
                    })
                }
            },
            None => String::from("At the bar 🍺"),
        };

        let teacher: String = match event["prof_inst"][0]["title"].as_str() {
            Some(prof_inst) => String::from(prof_inst),
            None => match event["title"].as_str() {
                Some(title) => String::from(title),
                None => String::from("No teacher"),
            },
        };

        let time_start: String = match event["start"].as_str() {
            Some(start) => match format::time(&start) {
                Some(start) => start,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("formatting value `start` failed"),
                    })
                }
            },
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `start` does not exist"),
                })
            }
        };

        let time_end: String = match event["end"].as_str() {
            Some(end) => match format::time(&end) {
                Some(end) => end,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("formatting value `end` failed"),
                    })
                }
            },
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `end` does not exist"),
                })
            }
        };

        let registration_status: bool = match event["event_registered"].as_str() {
            Some(event_registered) => {
                if event_registered == "registered" || event_registered == "present" {
                    true
                } else {
                    false
                }
            }
            None => match event["event_registered"].as_bool() {
                Some(event_unregistered) => {
                    if event_unregistered == false {
                        false
                    } else {
                        return HttpResponse::InternalServerError().json(data::Default {
                            msg: String::from("formatting value `event_registered` failed"),
                        });
                    }
                }
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `event_registered` does not exist"),
                    })
                }
            },
        };

        if is_rdv == true && registration_status == true {
            // TODO: get additional information for rdv events
        }

        // Push event into list
        list.push(data::PlanningDayResult {
            is_custom: is_custom,
            is_rdv: is_rdv,
            is_regular: is_regular,
            year: year,
            code_module: code_module,
            code_instance: code_instance,
            code_acti: code_acti,
            code_event: code_event,
            semester: semester_event,
            title: title,
            module: module,
            room: room,
            teacher: teacher,
            time_start: time_start,
            time_end: time_end,
            registration_status: registration_status,
        });
    }

    // TODO: get list of custom calendars with their events and add them to list

    HttpResponse::Ok().json(list)
}
