//{"name":"S3 - Kaguya Wants to Analyze the Academy Anthem","group":"DMOJ - Mock CCC '22 2","url":"https://dmoj.ca/problem/nccc10s3","interactive":false,"timeLimit":5000,"tests":[{"input":"abacabadabacaba\n4\na 7\ne 3\nbac 2\nabada 1\n","output":"13\n-1\n10\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S3KaguyaWantsToAnalyzeTheAcademyAnthem"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::hash::{hash, HashBase, SimpleHash, StringHash};
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let s: Str = input.read();
    let q = input.read_size();
    let queries: Vec<(Str, usize)> = input.read_vec(q);

    let res = solve_impl(s, queries);
    output().print_per_line(&res);
}

fn solve_impl(s: Str, queries: Vec<(Str, usize)>) -> Vec<Option<usize>> {
    let mut res = Vec::new();
    HashBase::init();
    let h = SimpleHash::new(s.as_slice());
    let mut ans: Vec<DefaultHashMap<i64, Vec<usize>>> = vec![DefaultHashMap::new(); s.len() + 1];
    for (sample, index) in queries {
        let index = index - 1;

        if sample.len() > s.len() {
            res.push(None);
            continue;
        }
        if ans[sample.len()].is_empty() {
            for i in 0..=s.len() - sample.len() {
                ans[sample.len()][h.hash(i..i + sample.len())].push(i + 1);
            }
        }
        let c_hash = hash(sample.as_slice());
        if ans[sample.len()][c_hash].len() <= index {
            res.push(None);
        } else {
            res.push(Some(ans[sample.len()][c_hash][index]));
        }
    }
    res
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

mod tester {
    use algo_lib::io::input::Input;
    use algo_lib::io::output::{Output, OUTPUT};

    fn check(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
        let mut expected = Input::new(expected);
        let mut actual = Input::new(actual);
        let mut token_num = 0usize;
        loop {
            let expected_token = expected.next_token();
            let actual_token = actual.next_token();
            if expected_token != actual_token {
                if expected_token.is_none() {
                    return Err(format!("Expected has only {} tokens", token_num));
                } else if actual_token.is_none() {
                    return Err(format!("Actual has only {} tokens", token_num));
                } else {
                    return Err(format!(
                        "Token #{} differs, expected {}, actual {}",
                        token_num,
                        String::from_utf8(expected_token.unwrap()).unwrap(),
                        String::from_utf8(actual_token.unwrap()).unwrap()
                    ));
                }
            }
            token_num += 1;
            if actual_token.is_none() {
                break;
            }
        }
        Ok(())
    }

    static mut OUT: Vec<u8> = Vec::new();

    struct WriteDelegate {}

    impl std::io::Write for WriteDelegate {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            unsafe {
                OUT.append(&mut Vec::from(buf));
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    pub(crate) fn run_tests() -> bool {
        let blue = "\x1B[34m";
        let red = "\x1B[31m";
        let green = "\x1B[32m";
        let yellow = "\x1B[33m";
        let def = "\x1B[0m";
        let time_limit = std::time::Duration::from_millis(5000);
        let mut paths = std::fs::read_dir("./tests/s3_kaguya_wants_to_analyze_the_academy_anthem/")
            .unwrap()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>();
        paths.sort_by(|a, b| a.path().cmp(&b.path()));
        let mut test_failed = 0usize;
        let mut test_total = 0usize;
        for path in paths {
            let sub_path = path;
            if sub_path.file_type().unwrap().is_file() {
                let path = sub_path.path();
                match path.extension() {
                    None => {}
                    Some(extension) => {
                        if extension.to_str() == Some("in") {
                            println!("=====================================================");
                            test_total += 1;
                            let name = path.file_name().unwrap().to_str().unwrap();
                            let name = &name[..name.len() - 3];
                            println!("{}Test {}{}", blue, name, def);
                            println!("{}Input:{}", blue, def);
                            println!("{}", std::fs::read_to_string(&path).unwrap());
                            let expected = match std::fs::read_to_string(
                                path.parent().unwrap().join(format!("{}.out", name)),
                            ) {
                                Ok(res) => Some(res),
                                Err(_) => None,
                            };
                            println!("{}Expected:{}", blue, def);
                            match &expected {
                                None => {
                                    println!("{}Not provided{}", yellow, def);
                                }
                                Some(expected) => {
                                    println!("{}", expected);
                                }
                            }
                            println!("{}Output:{}", blue, def);
                            match std::panic::catch_unwind(|| {
                                unsafe {
                                    OUT.clear();
                                }
                                let mut file = std::fs::File::open(&path).unwrap();
                                let input = Input::new(&mut file);
                                let started = std::time::Instant::now();
                                unsafe {
                                    OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
                                }
                                let is_exhausted = crate::run(input);
                                let res = started.elapsed();
                                let output;
                                unsafe {
                                    output = OUT.clone();
                                }
                                println!("{}", String::from_utf8_lossy(&output));
                                (output, res, is_exhausted)
                            }) {
                                Ok((output, duration, is_exhausted)) => {
                                    println!(
                                        "{}Time elapsed: {:.3}s{}",
                                        blue,
                                        (duration.as_millis() as f64) / 1000.,
                                        def,
                                    );
                                    if !is_exhausted {
                                        println!("{}Input not exhausted{}", red, def);
                                    }
                                    if let Some(expected) = expected {
                                        let mut expected_bytes = expected.as_bytes().clone();
                                        match check(&mut expected_bytes, &mut &output[..]) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                println!(
                                                    "{}Verdict: {}Wrong Answer ({}){}",
                                                    blue, red, err, def
                                                );
                                                test_failed += 1;
                                                continue;
                                            }
                                        }
                                    }
                                    if duration > time_limit {
                                        test_failed += 1;
                                        println!("{}Verdict: {}Time Limit{}", blue, red, def);
                                    } else {
                                        println!("{}Verdict: {}OK{}", blue, green, def)
                                    }
                                }
                                Err(err) => {
                                    test_failed += 1;
                                    match err.downcast::<&str>() {
                                        Ok(as_string) => println!(
                                            "{}Verdict: {}RuntimeError ({:?}){}",
                                            blue, red, as_string, def
                                        ),
                                        Err(err) => println!(
                                            "{}Verdict: {}RuntimeError ({:?}){}",
                                            blue, red, err, def
                                        ),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if test_failed == 0 {
            println!(
                "{}All {}{}{} tests passed{}",
                blue, green, test_total, blue, def
            );
        } else {
            println!(
                "{}{}/{}{} tests failed{}",
                red, test_failed, test_total, blue, def
            );
        }
        test_failed == 0
    }
}
#[test]
fn s3_kaguya_wants_to_analyze_the_academy_anthem() {
    assert!(tester::run_tests());
}
