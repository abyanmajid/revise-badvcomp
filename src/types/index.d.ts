import { NavItem } from "./nav"

export type SiteConfig = {
  name: string
  author: string
  description: string
  keywords: Array<string>
  url: {
    base: string
    author: string
  },
  mainNav: NavItem[]
  links: {
    github: string
    twitter: string
  }
  ogImage: string
}
