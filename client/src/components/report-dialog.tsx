import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Button } from "@/components/ui/button"
import { BugIcon } from "lucide-react"
import Link from "next/link"

export function ReportDialog() {
  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="secondary">Report Bug&nbsp;<BugIcon size={16} /></Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Found a bug?</AlertDialogTitle>
          <AlertDialogDescription>
            Please open an issue on GitHub by clicking the button below. Alternatively, please contact me on discord: <span className="text-primary font-bold">@kinderheim.511</span>
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <Link href="https://github.com/abyanmajid/revise-badvcomp/issues/new" target="_blank"><AlertDialogAction>Open an issue</AlertDialogAction></Link>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
