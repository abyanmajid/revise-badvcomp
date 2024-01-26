import Link from "next/link"
import React from "react"

import { siteConfig } from "@/config/site"
import { cn } from "@/lib/utils"
import { buttonVariants } from "@/components/ui/button"
import { ModeToggle } from "@/components/mode-toggle"
import { GitHubLogoIcon } from "@radix-ui/react-icons"
import { Separator } from "@/components/ui/separator"

export default function Home() {
  return (
    <main className="flex my-32 justify-center">
      <div className="container flex max-w-[64rem] flex-col items-center gap-4 text-center">
        <h1 className="text-4xl font-semibold sm:text-5xl md:text-6xl lg:text-7xl">
          {siteConfig.name}
        </h1>
        <p className="max-w-[42rem] leading-normal text-muted-foreground sm:text-xl sm:leading-8">
          Generate practice problems for <Link href="https://www.sydney.edu.au/courses/courses/uc/bachelor-of-advanced-computing.html" className="text-black dark:text-white hover:text-red-600 dark:hover:text-red-600">USYD&apos;s BAdvComp</Link> courses.
        </p>
        <Separator className="max-w-[42rem]" />
        <p className="mb-2 max-w-[42rem] leading-normal text-muted-foreground sm:text-xl sm:leading-8">
          Built by <Link href={siteConfig.url.author} className="text-black dark:text-white hover:text-red-600 dark:hover:text-red-600">{siteConfig.author}</Link>
        </p>
        <div className="flex gap-2">
          <Link
            href="/units"
            className={cn(buttonVariants({ size: "default" }))}
          >
            Start Revising
          </Link>
          <Link
            href={siteConfig.links.github}
            target="_blank"
            className={cn(buttonVariants({ size: "default", variant: "secondary" }))}
          >
            <GitHubLogoIcon />&nbsp; Source
          </Link>
          <ModeToggle />
        </div>
      </div>
    </main>
  )
}

