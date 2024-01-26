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
    syllabus: "Semester 2, 2023"
  },
]
