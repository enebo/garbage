use std::env;
use std::ffi::{OsString, OsStr};
use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use fnv::FnvHashMap;


lazy_static! {
  static ref HOME: OsString = OsString::from("HOME");
}

fn retreive_envs_as_osstring() -> HashMap<OsString, OsString> {
    env::vars_os().collect()
}

fn retreive_envs() ->  HashMap<String, String> {
    env::vars().collect::<HashMap<String, String>>()
}

fn retreive_envs_as_osstring_fnv() -> FnvHashMap<OsString, OsString> {
    env::vars_os().collect()
}

fn retreive_envs_fnv() ->  FnvHashMap<String, String> {
    env::vars().collect::<FnvHashMap<String, String>>()
}

fn compare_envs() -> bool {
    let envs = retreive_envs();

    if let Some(home) = envs.get("HOME") {
        home == "/home/enebo"
    } else {
        false
    }

}

fn compare_envs_as_osstring() -> bool {
    let envs = retreive_envs_as_osstring();

    if let Some(home) = envs.get(&HOME.to_os_string()) {
        home == "/home/enebo"
    } else {
        false
    }
}

fn compare_envs_fnv() -> bool {
    let envs = retreive_envs_fnv();

    if let Some(home) = envs.get("HOME") {
        home == "/home/enebo"
    } else {
        false
    }

}

fn compare_envs_as_osstring_fnv() -> bool {
    let envs = retreive_envs_as_osstring_fnv();

    if let Some(home) = envs.get(&HOME.to_os_string()) {
        home == "/home/enebo"
    } else {
        false
    }
}

fn bench_contains(c: &mut Criterion) {
    c.bench_function("ostrings", |b| b.iter(|| black_box(retreive_envs_as_osstring())));
    c.bench_function("strings", |b| b.iter(|| black_box(retreive_envs())));
    c.bench_function("ostrings (cmp)", |b| b.iter(|| black_box(compare_envs_as_osstring())));
    c.bench_function("strings (cmp)", |b| b.iter(|| black_box(compare_envs())));
    c.bench_function("ostrings (cmp, fnv)", |b| b.iter(|| black_box(compare_envs_as_osstring_fnv())));
    c.bench_function("strings (cmp, fnv)", |b| b.iter(|| black_box(compare_envs_fnv())));
}

criterion_group!(benches, bench_contains);
criterion_main!(benches);