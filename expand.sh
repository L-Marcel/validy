#!/bin/bash

set -e
mkdir -p temp
mkdir -p temp/validations
mkdir -p temp/modifications

cargo expand --all-features --test mod validations::allowlist >> temp/validations/allowlist.rs || true
cargo expand --all-features --test mod validations::blocklist >> temp/validations/blocklist.rs || true
cargo expand --all-features --test mod validations::contains >> temp/validations/contains.rs || true
cargo expand --all-features --test mod validations::email >> temp/validations/email.rs || true
cargo expand --all-features --test mod validations::ip >> temp/validations/ip.rs || true
cargo expand --all-features --test mod validations::ipv4 >> temp/validations/ivp4.rs || true
cargo expand --all-features --test mod validations::ipv6 >> temp/validations/ipv6.rs || true
cargo expand --all-features --test mod validations::length >> temp/validations/length.rs || true
cargo expand --all-features --test mod validations::range >> temp/validations/range.rs || true
cargo expand --all-features --test mod validations::prefix >> temp/validations/prefix.rs || true
cargo expand --all-features --test mod validations::suffix >> temp/validations/suffix.rs || true
cargo expand --all-features --test mod validations::url >> temp/validations/url.rs || true
cargo expand --all-features --test mod validations::pattern >> temp/validations/pattern.rs || true
cargo expand --all-features --test mod validations::option >> temp/validations/option.rs || true
cargo expand --all-features --test mod validations::time >> temp/validations/time.rs || true
cargo expand --all-features --test mod validations::naive_time >> temp/validations/naive_time.rs || true
cargo expand --all-features --test mod validations::naive_date >> temp/validations/naive_date.rs || true
cargo expand --all-features --test mod validations::after_now >> temp/validations/after_now.rs || true
cargo expand --all-features --test mod validations::before_now >> temp/validations/before_now.rs || true
cargo expand --all-features --test mod validations::now >> temp/validations/now.rs || true
cargo expand --all-features --test mod validations::today >> temp/validations/today.rs || true
cargo expand --all-features --test mod validations::after_today >> temp/validations/after_today.rs || true
cargo expand --all-features --test mod validations::before_today >> temp/validations/before_today.rs || true
cargo expand --all-features --test mod validations::inline >> temp/validations/inline.rs || true
cargo expand --all-features --test mod validations::custom >> temp/validations/custom.rs || true
cargo expand --all-features --test mod validations::async_custom >> temp/validations/async_custom.rs || true
cargo expand --all-features --test mod validations::async_custom_with_context >> temp/validations/async_custom_with_context.rs || true
cargo expand --all-features --test mod validations::custom_with_context >> temp/validations/custom_with_context.rs || true

cargo expand --all-features --test mod modifications::trim >> temp/modifications/trim.rs || true
cargo expand --all-features --test mod modifications::trim_start >> temp/modifications/trim_start.rs || true
cargo expand --all-features --test mod modifications::trim_end >> temp/modifications/trim_end.rs || true
cargo expand --all-features --test mod modifications::uppercase >> temp/modifications/uppercase.rs || true
cargo expand --all-features --test mod modifications::lowercase >> temp/modifications/lowercase.rs || true
cargo expand --all-features --test mod modifications::capitalize >> temp/modifications/capitalize.rs || true
cargo expand --all-features --test mod modifications::camel_case >> temp/modifications/camel_case.rs || true
cargo expand --all-features --test mod modifications::lower_camel_case >> temp/modifications/lower_camel_case.rs || true
cargo expand --all-features --test mod modifications::snake_case >> temp/modifications/snake_case.rs || true
cargo expand --all-features --test mod modifications::shouty_snake_case >> temp/modifications/shouty_snake_case.rs || true
cargo expand --all-features --test mod modifications::kebab_case >> temp/modifications/kebab_case.rs || true
cargo expand --all-features --test mod modifications::shouty_kebab_case >> temp/modifications/shouty_kebab_case.rs || true
cargo expand --all-features --test mod modifications::train_case >> temp/modifications/train_case.rs || true
cargo expand --all-features --test mod modifications::parse_naive_date >> temp/modifications/parse_naive_date.rs || true
cargo expand --all-features --test mod modifications::parse_naive_time >> temp/modifications/parse_naive_time.rs || true
cargo expand --all-features --test mod modifications::parse_time >> temp/modifications/parse_time.rs || true

cargo check --all-features --test mod
