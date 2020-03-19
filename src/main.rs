#[derive(Debug, Clone, Eq, PartialEq)]
enum RulePart {
	Terminal(String),
	NonTerminal(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
	name: String,
	tokens: Vec<RulePart>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State<'a> {
	rule: &'a Rule,
	rule_pos: usize,
	start_from: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct StateSet<'a>(Vec<State<'a>>);

impl<'a> State<'a> {
	fn next_element(&self) -> Option<&RulePart> {
		self.rule.tokens.get(self.rule_pos)
	}
}

impl<'a> StateSet<'a> {
	fn add_to_set(&mut self, state: State<'a>) {
		if self.0.iter().find(|&x| *x == state).is_none() {
			self.0.push(state);
		}
	}
}

use std::fmt;

impl fmt::Display for State<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({} ->", self.rule.name)?;
		for (index, i) in self.rule.tokens.iter().enumerate() {
			if index == self.rule_pos {
				write!(f, " •")?;
			}
			match i {
				RulePart::Terminal(t) => write!(f, " {:?}", t)?,
				RulePart::NonTerminal(nt) => write!(f, " {}", nt)?,
			}
		}
		if self.rule_pos == self.rule.tokens.len() {
			write!(f, " •")?;
		}
		write!(f, ", {})", self.start_from)?;
		Ok(())
	}   
}

fn parse<'a>(tokens: &[String], grammar: &'a [Rule]) -> Option<Vec<StateSet<'a>>> {
	if grammar.is_empty() {
		return None;
	}
	if tokens.is_empty() {
		return None;
	}
	let mut s: Vec<StateSet> = vec![StateSet(vec![]); tokens.len()+1];

	s.get_mut(0)?.0.push(State { 
		rule: &grammar[0],
		rule_pos: 0,
		start_from: 0,
	});

	let eof = "eof".to_string();
	for (k, token) in tokens.iter().chain((0..1).map(|_| &eof)).enumerate() {
		let mut i = 0;
		while i < s[k].0.len() {
			let state = s[k].0[i].clone();
			match state.next_element() {
				Some(RulePart::NonTerminal(nt)) => {
					// Predictor
					for g in grammar.iter().filter(|&x| x.name == *nt) {
						s[k].add_to_set(State {
							rule: g,
							rule_pos: 0,
							start_from: k,
						});
					}
				},
				Some(RulePart::Terminal(t)) => {
					if t == token {
						s[k+1].add_to_set(State {
							rule: state.rule,
							rule_pos: state.rule_pos+1,
							start_from: state.start_from,
						});
					}
				},
				None => {
					// Completer
					let name = &state.rule.name;
					let pos = state.start_from;

					let mut j = 0;
					while j < s[pos].0.len() {
						if let Some(RulePart::NonTerminal(nt)) = s[pos].0[j].next_element() {
							if name == nt {
								let current_state = s[pos].0[j].clone();
								s[k].add_to_set(State {
									rule: current_state.rule,
									rule_pos: current_state.rule_pos+1,
									start_from: current_state.start_from,
								});
							}
						}
						j += 1;
					}
				}
			}
			i += 1;
		}
	}

	let mut flag = false;
	for state in &s.last()?.0 {
		if *state.rule == grammar[0] && state.next_element().is_none() {
			flag = true;
		}
	}

	for (index, i) in s.iter().enumerate() {
		println!("[{}]", index);
		for state in &i.0 {
			println!("{}", state);
		}
		println!();
	}

	if flag {
		Some(s)	
	} else {
		None
	}
}

fn main() {
	macro_rules! parse {
		($x:ident) => {
			RulePart::NonTerminal(stringify!($x).to_string())
		};

		($x:literal) => {
			RulePart::Terminal($x.to_string())
		};
	}

	macro_rules! grammar_rule {
		($name:ident -> $($other:tt)*) => {{
			let mut result = Rule {
				name: stringify!($name).to_string(),
				tokens: vec![],
			};
			$(
				result.tokens.push(parse!($other));
			)*
			result
		}};
	}

	macro_rules! grammar {
		($($a:ident -> ($($x:tt)*));* $(;)?) => {{
			let mut result = Vec::new();
			$(
				result.push(grammar_rule!($a -> $($x)*));
			)*
			result
		}};
	}

	let grammar: Vec<Rule> = grammar! {
		P -> (S);
		S -> (S "+" M);
		S -> (M);
		M -> (M "*" T);
		M -> (T);
		T -> ("1");
		T -> ("2");
		T -> ("3");
		T -> ("4");
	};
	dbg!(&grammar);
	let tokens: Vec<String> = "2+3*4+".chars().map(|c| c.to_string()).collect();
	dbg!(&tokens);
	let result = parse(&tokens, &grammar);
	dbg!(result);
}
