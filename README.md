# Revise BAdvComp

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/abyanmajid/revise-badvcomp/blob/main/LICENSE) ![ci-badge](https://github.com/abyanmajid/revise-badvcomp/actions/workflows/ci.yml/badge.svg) ![Rust](https://img.shields.io/badge/Axum-red.svg?style=flat&logo=rust&logoColor=white) ![Next JS](https://img.shields.io/badge/Next-black?style=flat&logo=next.js&logoColor=white)

Public API written in Rust for generating practice problems to help in revising for USYD's Bachelor of Advanced Computing Units.

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

To generate a practice problem, run
```
curl https://revise-badvcomp-bhfzgseo6q-uc.a.run.app/elec1601/<topic_id>/<question_type_id> | jq
```

## Tech stack

- API server: ***Axum*** <sub>(Rust)</sub>
- User interface: ***Next.js*** <sub>(TypeScript)</sub>
- Tracing, server error handling: ***Clap*** <sub>(Rust)</sub>, ***Tracing*** <sub>(Rust)</sub>, ***Anyhow*** <sub>(Rust)</sub>
- Containerization, CI/CD: ***Docker***, ***GitHub Actions***
- Deployment: ***GCP Cloud Run***, ***Vercel***
