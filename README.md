# Revise BAdvComp

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/abyanmajid/revise-badvcomp/blob/main/LICENSE) ![ci-badge](https://github.com/abyanmajid/revise-badvcomp/actions/workflows/ci.yml/badge.svg) ![Rust](https://img.shields.io/badge/Axum-red.svg?style=flat&logo=rust&logoColor=white) ![Next JS](https://img.shields.io/badge/Next-black?style=flat&logo=next.js&logoColor=white)

Public REST API written in Tokio-Axum (Rust) with NextJS user interface for generating practice problems based on randomness in hopes to help in revising for USYD's Bachelor of Advanced Computing units.

Live app at **[revise-badvcomp.vercel.app](https://revise-badvcomp.vercel.app)**

## Contributing

To contribute in ***Rust***, submit a pull request with your changes in `server/src/<unit_code>`.

To contribute in ***other languages***, submit a pull request with a new file(s) in `contributions/` containing your generation logic. I will translate your code into Rust.

***Accepted languages***: Rust, Python, Java, JavaScript/TypeScript, C, C#, C++, Go

### Code structure

Please name your file in the format `<unit_code>-<file_name>`, and your generation functions must return a `(question: string, correct_answer: string)` tuple.
```
function generate_problem -> (string, string)
  // generation logic

  return (question, correct_answer)
```

See `contributions/unit0000-example.c` for example.

## License

Revise BAdvComp is MIT-licensed.
