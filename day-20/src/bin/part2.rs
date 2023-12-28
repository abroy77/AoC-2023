use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (_, modules) = parse_input(&contents).unwrap();
    let sol = get_solution(modules);

    println!("Solution: {}", sol);
}
#[derive(Clone, Debug)]
enum ModuleType {
    FF { state: bool },
    CON { states: HashMap<String, PulseType> },
    BC,
}
#[derive(Clone, Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PulseType {
    Low,
    High,
}

#[derive(Clone, Debug)]
struct Pulse {
    pulse_type: PulseType,
    origin: String,
    destination: String,
}

impl PulseType {
    fn new() -> Self {
        PulseType::Low
    }

    fn flip(&mut self) {
        // mutate self in place
        match self {
            PulseType::Low => *self = PulseType::Low,
            PulseType::High => *self = PulseType::High,
        }
    }
}
impl<'a> Module {
    fn relay(&mut self, p: PulseType, origin: String) -> Vec<Pulse> {
        match &mut self.module_type {
            ModuleType::BC => self
                .destinations
                .iter()
                .map(|d| Pulse {
                    pulse_type: p,
                    origin: self.name.clone(),
                    destination: d.clone(),
                })
                .collect(),
            ModuleType::FF { state } => {
                if p == PulseType::Low {
                    *state = !*state;
                    let p_type = if *state {
                        PulseType::High
                    } else {
                        PulseType::Low
                    };
                    self.destinations
                        .iter()
                        .map(|d| Pulse {
                            pulse_type: p_type,
                            origin: self.name.clone(),
                            destination: d.clone(),
                        })
                        .collect()
                } else {
                    vec![]
                }
            }

            ModuleType::CON { states } => {
                states.entry(origin).and_modify(|origin_state| {
                    *origin_state = p;
                });
                let all_high = states.values().all(|v| *v == PulseType::High);
                let pulse_type = if all_high {
                    PulseType::Low
                } else {
                    PulseType::High
                };

                self.destinations
                    .iter()
                    .map(|d| Pulse {
                        pulse_type,
                        origin: self.name.clone(),
                        destination: d.clone(),
                    })
                    .collect()
            }
        }
    }
}

// fn relay<'a, 'b, 'c>(module: &mut Module<'a>, p: PulseType, origin: String) -> Vec<Pulse<'a>> {
//     match module.module_type {
//         ModuleType::BC => module
//             .destinations
//             .iter()
//             .map(|d| Pulse {
//                 pulse_type: p,
//                 origin: module.name,
//                 destination: d,
//             })
//             .collect(),
//         ModuleType::FF { mut state } => {
//             if p == PulseType::Low {
//                 state = !state;
//                 let p_type = if state {
//                     PulseType::High
//                 } else {
//                     PulseType::Low
//                 };
//                 module
//                     .destinations
//                     .iter()
//                     .map(|d| Pulse {
//                         pulse_type: p_type,
//                         origin: module.name,
//                         destination: d,
//                     })
//                     .collect()
//             } else {
//                 vec![]
//             }
//         }

//         ModuleType::CON { mut states } => {
//             states.insert(origin, p.clone());
//             let all_high = states.iter().all(|(_, v)| *v == PulseType::High);
//             let pulse_type = if all_high {
//                 PulseType::Low
//             } else {
//                 PulseType::High
//             };

//             module
//                 .destinations
//                 .iter()
//                 .map(|d| Pulse {
//                     pulse_type,
//                     origin: module.name,
//                     destination: d,
//                 })
//                 .collect()
//         }
//     }
// }

fn get_solution<'a>(mut modules: HashMap<String, Module>) -> usize {
    // we need to get the button presses it takes to get to the feeders of the
    // feeder to rx.
    // as explained in: https://www.youtube.com/watch?v=lxm6i21O83k&ab_channel=HyperNeutrino
    // and then find the LCM with the assumption that the button presses to
    // get to each feeder is cyclic

    let feeder = modules
        .values()
        .find(|m| m.destinations.contains(&"rx".to_string()))
        .unwrap()
        .name
        .clone();

    println!("feeder: {}", feeder);

    let feedfeed_names = modules
        .values()
        .filter(|m| m.destinations.contains(&feeder))
        .map(|m| m.name.clone())
        .collect::<Vec<String>>();

    println!("feedfeed_names: {:?}", feedfeed_names);

    let mut feedfeeds: HashMap<String, usize> =
        feedfeed_names.iter().map(|m| (m.clone(), 0)).collect();

    println!("feedfeeds: {:?}", feedfeeds);

    // .map(|m| (m.name.clone(), 0))
    // .collect::<HashMap<String, usize>>();
    let mut button_presses = 0;

    'button: loop {
        button_presses += 1;
        let mut pulses: VecDeque<Pulse> = VecDeque::new();
        // add start pulse to deque
        pulses.push_back(Pulse {
            pulse_type: PulseType::Low,
            origin: "button".to_string(),
            destination: "broadcaster".to_string(),
        });
        while let Some(Pulse {
            pulse_type,
            origin,
            destination,
        }) = pulses.pop_front()
        {
            // check if rx is activated

            let new_pulses: Vec<Pulse>;
            if let Some(module) = modules.get_mut(&destination) {
                new_pulses = module.relay(pulse_type, origin);
                if feedfeed_names.contains(&destination)
                    && new_pulses.iter().any(|p| p.pulse_type == PulseType::High)
                {
                    println!("{} has a low pulse", destination);
                    println!("button_presses: {}", button_presses);
                    feedfeeds
                        .entry(destination)
                        .and_modify(|v| *v = button_presses);
                }

                pulses.extend(new_pulses);
            }

            // check if a feedfeed module as output a low pulse
        }

        if feedfeeds.values().all(|v| *v > 0) {
            break 'button;
        }
    }
    feedfeeds.values().fold(1, |acc, v| lcm(acc, *v))
}

fn gcd(a: usize, b: usize) -> usize {
    // make sure a is the larger number
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn parse_line(input: &str) -> IResult<&str, Module> {
    let (input, ((mod_type_char, mod_name), destinations)) = separated_pair(
        tuple((anychar, alpha1)),
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(input)?;
    let mod_type = match mod_type_char {
        '%' => ModuleType::FF { state: false },
        '&' => ModuleType::CON {
            states: HashMap::new(),
        },
        'b' => ModuleType::BC,
        _ => panic!("Invalid module type"),
    };

    let mod_name_string: String;
    if mod_name == "roadcaster" {
        mod_name_string = "broadcaster".to_string();
    } else {
        mod_name_string = mod_name.to_string();
    }

    let dest_strings = destinations.iter().map(|d| d.to_string()).collect();

    let module = Module {
        name: mod_name_string,
        module_type: mod_type,
        destinations: dest_strings,
    };
    Ok((input, module))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, Module>> {
    let (input, mut modules) = separated_list1(tag("\n"), parse_line)(input)?;
    // get module and destinations as hashmap
    let mut mod_map: HashMap<String, Vec<String>> = HashMap::new();
    for module in modules.iter() {
        mod_map.insert(module.name.clone(), module.destinations.clone());
    }

    // for the CON modules, add the origins
    modules.iter_mut().for_each(|m| {
        if let ModuleType::CON { states } = &mut m.module_type {
            for (k, v) in mod_map.iter() {
                if v.contains(&m.name) {
                    states.insert(k.clone(), PulseType::Low);
                }
            }
        }
    });
    let mut mod_map = HashMap::new();

    modules.into_iter().for_each(|m| {
        mod_map.insert(m.name.clone(), m);
    });

    Ok((input, mod_map))
}
