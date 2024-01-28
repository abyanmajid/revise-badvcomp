import { SiteConfig } from "@/types"

import { env } from "@/env.mjs"

export const siteConfig: SiteConfig = {
  name: "Revise BAdvComp",
  author: "Abyan Majid",
  description:
    "Practice problems generator for USYD's Bachelor of Advanced Computing units",
  keywords: ["USYD", "Bachelor of Advanced Computing", "Computer Science"],
  url: {
    base: env.NEXT_PUBLIC_APP_URL,
    author: "https://www.github.com/abyanmajid",
  },
  mainNav: [
    {
      title: "Home",
      href: "/",
    },
    {
      title: "Units",
      href: "/units"
    }
  ],
  links: {
    github: "https://github.com/abyanmajid/revise-badvcomp",
  },
  ogImage: `${env.NEXT_PUBLIC_APP_URL}/og.jpg`,
}

export const UNITS = [
  {
    code: "elec1601",
    long: "Introduction to Computer Systems",
    syllabus: "Semester 2, 2023",
    topics: [
      {
        "id": 1,
        "topic": "Base encoding",
        "question_types": []
      },
      {
        "id": 2,
        "topic": "Fixed-point numbers",
        "question_types": [
          {
            "id": 1,
            "qtype": "Convert fixed-point to decimal"
          },
          {
            "id": 2,
            "qtype": "Convert decimal to fixed-point"
          }
        ]
      },
      {
        "id": 3,
        "topic": "Memory size",
        "question_types": [
          {
            "id": 1,
            "qtype": "Calculate memory size"
          },
          {
            "id": 2,
            "qtype": "Calculate cell size"
          },
          {
            "id": 3,
            "qtype": "Calculate number of cells"
          },
          {
            "id": 4,
            "qtype": "Calculate minimal address bits"
          }
        ]
      },
      {
        "id": 4,
        "topic": "Registers",
        "question_types": [
          {
            "id": 1,
            "qtype": "Trace registers value based on clock"
          }
        ]
      },
      {
        "id": 5,
        "topic": "Stack",
        "question_types": [
          {
            "id": 1,
            "qtype": "Keeping track of push and pop operations"
          }
        ]
      },
      {
        "id": 6,
        "topic": "AVR Assembly",
        "question_types": [
          {
            "id": 1,
            "qtype": "Bitwise AND + Reading Opcode"
          }
        ]
      }
    ]
  },
]
