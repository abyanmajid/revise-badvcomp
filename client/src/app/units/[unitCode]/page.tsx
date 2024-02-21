import { Separator } from "@/components/ui/separator"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import Generator from "@/components/generator"
import { UNITS } from "@/config/site"
import { notFound } from "next/navigation"
import { fetchData } from "@/lib/actions"
import { ReportDialog } from "@/components/report-dialog"

interface Params {
  params: {
    unitCode: string;
  }
}

export default async function AllUnitsPage({ params }: Params) {
  let unitCode = params.unitCode;
  const api_endpoint = `${process.env.SERVER_URL}/${unitCode}`;
  const unit = UNITS.find(unit => unit.code === unitCode);
  if (unit === undefined) {
    notFound()
  }
  unitCode = unitCode === "math1061-1021" ? "math1061 / 1021" : unitCode
  unitCode = unitCode === "math1061-1002" ? "math1061 / 1002" : unitCode

  return <main className="flex mt-8 justify-center">
    <div className="container flex max-w-[64rem] flex-col gap-4 text-center">
      <Card>
        <CardHeader className="parent text-left grid grid-cols-2">
          <div>
            <CardTitle className="text-2xl font-semibold sm:text-2xl md:text-3xl lg:text-4xl">
              {unitCode.toUpperCase()}
            </CardTitle>
            <CardDescription className="leading-normal text-muted-foreground md:text-lg sm:leading-8">
              <span className="font-semibold">
                {unit.long}&nbsp;
                {unitCode === "math1061 / 1021" ? "(Calculus)" : ""}
                {unitCode === "math1061 / 1002" ? "(Linear Algebra)" : ""}
              </span> <br />
              {unit.syllabus}
            </CardDescription>
          </div>
          <div className="flex justify-end">
            <ReportDialog />
          </div>
        </CardHeader>
        <Separator />
        <CardContent>
          <Generator api_endpoint={api_endpoint} topics={unit.topics} />
        </CardContent>
      </Card>
    </div>
  </main >
}
