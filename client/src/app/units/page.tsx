import Link from "next/link"
import { Separator } from "@/components/ui/separator"
import { Button } from "@/components/ui/button"

export default function AllUnitsPage() {
  return <main className="flex mt-8 justify-center">
    <div className="container flex max-w-[64rem] flex-col gap-4 md:text-start text-center">
      <div className="grid md:grid-cols-2">
        <h1 className="text-2xl font-semibold sm:text-2xl md:text-3xl lg:text-4xl">
          📖 All Units
        </h1>
        <div className="flex justify-center md:justify-end items-end">
          <p className="mt-2 max-w-[42rem] leading-normal text-muted-foreground md:text-lg sm:leading-8">
            What would you like to study?
          </p>
        </div>
      </div>
      <Separator className="max-w-[64rem]" />
      <div className="grid grid-cols-3">
        <Link href="/units/elec1601" className="mx-1"><Button variant="outline" className="w-full h-24 text-lg">ELEC1601</Button></Link>
        <Link href="/units/math1061-1002" className="mx-1"><Button variant="outline" className="w-full h-24 text-lg">MATH1061 / 1002</Button></Link>
        <Link href="/units/math1061-1021" className="mx-1"><Button variant="outline" className="w-full h-24 text-lg">MATH1061 / 1021</Button></Link>
      </div>
    </div>
  </main >
}
