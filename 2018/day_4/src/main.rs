extern crate chrono; // 0.4.6

use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

use chrono::offset::Utc;
use chrono::prelude::*;
use chrono::DateTime;

#[derive(Debug, Eq)]
struct DateInstruction {
    datetime: DateTime<Utc>,
    instructions: Vec<String>,
}

impl PartialOrd for DateInstruction {
    fn partial_cmp(&self, other: &DateInstruction) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for DateInstruction {
    fn cmp(&self, other: &DateInstruction) -> cmp::Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialEq for DateInstruction {
    fn eq(&self, other: &DateInstruction) -> bool {
        self.datetime == other.datetime
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GuardState {
    Sleep,
    Awake,
}

#[derive(Debug)]
struct SleepAwake {
    state: GuardState,
    time: DateTime<Utc>,
}

#[derive(Debug)]
struct Guard {
    id: u32,
    times: Vec<SleepAwake>,
    current_state: GuardState,
    minute_map: HashMap<DateTime<Utc>, u32>,
    minutes_asleep: u32,
}

impl Guard {
    pub fn new(new_id: u32) -> Self {
        Guard {
            id: new_id,
            times: Vec::new(),
            current_state: GuardState::Awake,
            minute_map: HashMap::new(),
            minutes_asleep: 0,
        }
    }
}

fn parse_input(input: String) -> Result<(HashMap<u32,i64>,HashMap<u32,HashMap<u32,u32>>), Box<Error>> {
    //magic for parsing input to a format I need
    let mut sorted_dates: Vec<DateInstruction> = Vec::new();

    let puzzle_input: Vec<&str> = input.split("\n").collect();

    for line in puzzle_input.iter() {
        let mut date_and_instructions: Vec<&str> = line.split(" ").collect();
        let ymd: Vec<&str> = date_and_instructions.remove(0).split("-").collect();
        let year = &ymd[0][1..].parse::<i32>()?;
        let month = ymd[1].parse::<u32>()?;
        let day = ymd[2].parse::<u32>()?;
        let hm: Vec<&str> = date_and_instructions.remove(0).split(":").collect();
        let hour = hm[0].parse::<u32>()?;
        let minute = &hm[1][..2].parse::<u32>()?;
        let second = 0;
        let this_date = Utc.ymd(*year, month, day).and_hms(hour, *minute, second);

        let mut remaining_instructions: Vec<String> = Vec::new();

        for ins in date_and_instructions.iter() {
            remaining_instructions.push(ins.to_string());
        }

        let date_instruction = DateInstruction {
            datetime: this_date,
            instructions: remaining_instructions,
        };

        sorted_dates.push(date_instruction);
    }

    sorted_dates.sort(); //newest time at top so will need a reverse iterator
    
    let mut guards: Vec<Guard> = Vec::new();

    if sorted_dates[sorted_dates.len() - 1].instructions[0] != "Guard" {
        return Err(From::from(
            "Error: instructions started without a guard! Check dates are sorted right",
        ));
    }
    let last_instruction = sorted_dates.len() - 1;
    //removing first instruction to set guard
    let ins = sorted_dates.remove(last_instruction);

    //geting the first guard and setting their start time to awake and the time their shift begins
    let mut on_guard = Guard::new(ins.instructions[1][1..].parse::<u32>()?);
    on_guard.times.push(SleepAwake {
        state: GuardState::Awake,
        time: ins.datetime,
    });

    //reverse this vector so we start at the first day at the oldest time
    for instruction in sorted_dates.iter().rev() {
        //still have to parse the instructions
        //check for the word guard, if so we flip the old guard to awake and end his shift
        //if not guard do as the instructions say
        if instruction.instructions[0] == "Guard" {
            guards.push(on_guard);
            on_guard = Guard::new(instruction.instructions[1][1..].parse::<u32>()?);
            on_guard.times.push(SleepAwake {
                state: GuardState::Awake,
                time: instruction.datetime,
            });
        } else if instruction.instructions[0] == "wakes" {
            match on_guard.current_state {
                GuardState::Awake => {
                    return Err(From::from("Error: Guard waking up when already WOKE"));
                }
                GuardState::Sleep => {
                    on_guard.current_state = GuardState::Awake;
                    on_guard.times.push(SleepAwake {
                        state: GuardState::Awake,
                        time: instruction.datetime,
                    });
                }
            }
        } else if instruction.instructions[0] == "falls" {
            match on_guard.current_state {
                GuardState::Sleep => {
                    return Err(From::from("Error: Guard waking up when already SLEPY"));
                }
                GuardState::Awake => {
                    on_guard.current_state = GuardState::Sleep;
                    on_guard.times.push(SleepAwake {
                        state: GuardState::Sleep,
                        time: instruction.datetime,
                    });
                }
            }
        } else {
            return Err(From::from("Error: unrecognized instruction, failure!"));
        }
    }

    //push the last guard in since we only push when we see the next guard
    guards.push(on_guard);

    let mut guard_asleep: HashMap<u32, i64> = HashMap::new();
    let mut minutes_asleep: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    for guard in guards.iter() {
        let id = guard.id;

        let mut guard_awake = false;
        let mut guard_sleep = false;
        let mut awake_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);
        let mut sleep_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc);

        for time in guard.times.iter() {
            if !guard_awake && !guard_sleep && time.state == GuardState::Awake {
                //this is the first of this time
                guard_awake = true;
                //println!("Guard {} starts at {}", id, time.time);
            } else if guard_awake && time.state == GuardState::Sleep {
                //println!("guard {} has fallen asleep at {}", id, time.time);

                guard_awake = false;
                guard_sleep = true;

                sleep_time = time.time;
            } else if !guard_awake && guard_sleep && time.state == GuardState::Awake {
                //println!("Guard {} woke up at {}", id, time.time);
                awake_time = time.time;

                //println!(
                //    "On gaurd #{}, adding {} minutes",
                //    guard.id,
                //    awake_time.signed_duration_since(sleep_time).num_minutes()
                //);
                let asleep_duration = awake_time.signed_duration_since(sleep_time).num_minutes();
                let g = guard_asleep.entry(id).or_insert(0);
                *g += asleep_duration;

                let start_minute = sleep_time.format("%M").to_string().parse::<i64>()?;
                //println!("TIME RANGE TEST: \n {:#?}", start_minute..start_minute+asleep_duration);

                for m in start_minute..start_minute+asleep_duration {
                    let this_g = minutes_asleep.entry(id).or_insert(HashMap::new());
                    ////println!("{}", this_g);
                    let am = this_g.entry(m as u32).or_insert(0);
                        *am+=1;
                } 

                guard_awake = true;
                guard_sleep = false;
            } else {
            }
        }
    }

    Ok((guard_asleep,minutes_asleep))
}

//more functions for solving the problem!
fn strategy_one(guard_asleep: &HashMap<u32,i64>, minutes_asleep: &HashMap<u32, HashMap<u32,u32>>) -> Result<u32, Box<Error>> {
    //first strategy
    //take the guard that is most alseep
    //and the minute that the guard spent the most asleep
    //multiple them and return the function

    let mut sleepest_guard = 0;
    let mut sleepest_guard_minutes = 0;


    for (k,v) in guard_asleep.iter() {
        if v > &sleepest_guard_minutes {
            sleepest_guard_minutes = *v;
            sleepest_guard = *k;
        }
    }

    let mut sleepest_minute_check = 0;
    let mut sleepest_minutes = 0;

    let sleepy_minutes: &HashMap<u32, u32> = minutes_asleep.get(&sleepest_guard).unwrap(); 

    for (k,v) in sleepy_minutes.iter() {
        if v > &sleepest_minute_check {
            sleepest_minute_check = *v;
            sleepest_minutes = *k;
        }
    }


    //println!("Sleepy Guard: {:#?}", sleepest_guard);
    //println!("Minutes Asleep: {:#?}", sleepest_minutes);
    Ok(sleepest_guard*sleepest_minutes as u32)
}

fn strategy_two(minutes_asleep: &HashMap<u32, HashMap<u32,u32>>) -> Result<u32, Box<Error>> {
    let mut guard_asleep = 0;
    let mut most_asleep_minute = 0;
    let mut sleep_time = 0;

    for (guard_id,minute_map) in minutes_asleep.iter() {
        for (minute,minute_track) in minute_map.iter() {
            if minute_track > &sleep_time {
                guard_asleep = *guard_id;
                most_asleep_minute = *minute;
                sleep_time = *minute_track;
            }
        }
    }

    Ok(guard_asleep * most_asleep_minute)
}

fn main() -> Result<(), Box<Error>> {
        let mut puzzle_input = String::new();
            stdin().read_to_string(&mut puzzle_input)?;

 /*   let puzzle_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";*/

    let (guards,minutes) = parse_input(puzzle_input.to_string())?;
    

    println!("Part one: {}", strategy_one(&guards,&minutes)?);
    println!("Part two: {}", strategy_two(&minutes)?);

    Ok(())
}
