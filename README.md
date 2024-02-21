# Revise BAdvComp

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/abyanmajid/revise-badvcomp/blob/main/LICENSE) ![ci-badge](https://github.com/abyanmajid/revise-badvcomp/actions/workflows/ci.yml/badge.svg) ![Rust](https://img.shields.io/badge/Axum-red.svg?style=flat&logo=rust&logoColor=white) ![Next JS](https://img.shields.io/badge/Next-black?style=flat&logo=next.js&logoColor=white)

Public API written in Tokio-Axum (Rust) with NextJS UI for generating practice problems with random numbers to help in revising for USYD's Bachelor of Advanced Computing Units.

Live app at **[revise-badvcomp.vercel.app](https://revise-badvcomp.vercel.app)**

## Making `GET` requests

Install the `jq` package in order to format JSON data.
```
sudo apt-get install jq
```
To fetch available units, run
```
curl https://revise-badvcomp-bhfzgseo6q-uc.a.run.app | jq
```

To fetch available topics, run
```
curl https://revise-badvcomp-bhfzgseo6q-uc.a.run.app/<unit_code> | jq
```

To fetch a practice problem, run
```
curl https://revise-badvcomp-bhfzgseo6q-uc.a.run.app/elec1601/<topic_id>/<question_type_id> | jq
```

## Contributing

To contribute in ***Rust***, submit a pull request with your changes in `server/src/<unit_code>`.

To contribute in ***other languages***, submit a pull request with a new file(s) in `contributions/` containing your generation logic. I will translate your code into Rust.

***Accepted languages***: Rust, Python, Java, JavaScript/TypeScript, C, C#, C++, Go

## License

Revise BAdvComp is MIT-licensed.
