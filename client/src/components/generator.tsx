"use client";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuRadioItem,
  DropdownMenuRadioGroup,
} from "@/components/ui/dropdown-menu"
import {
  Card,
  CardContent,
  CardHeader,
  CardDescription,
} from "@/components/ui/card"
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { parseStringToArray } from "@/lib/utils"
import { Topic } from "@/types/units";

interface Props {
  api_endpoint: string;
  topics: Topic[];
}

export default function Generator({ api_endpoint, topics }: Props) {
  const [topic, setTopic] = useState(["Any", 0])
  const [questionType, setQuestionType] = useState(["Any", 0])
  const [question, setQuestion] = useState("")
  const [answer, setAnswer] = useState("")
  const [questionGenerated, setQuestionGenerated] = useState(false)
  const [answerRevealed, setAnswerRevealed] = useState(false)

  function handleTopicChange(newValue: string) {
    const newTopic = parseStringToArray(newValue)
    setTopic(newTopic)
    setQuestionType(["Any", 0])
  }

  function handleQuestionTypeChange(newValue: string) {
    const newQuestionType = parseStringToArray(newValue)
    setQuestionType(newQuestionType)
  }

  async function handleGenerate() {
    try {
      const response = await fetch(`${api_endpoint}/${topic[1]}/${questionType[1]}`);
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      const data = await response.json();
      setQuestion(data.question)
      setAnswer(data.answer)
      setQuestionGenerated(true)
      setAnswerRevealed(false)
    } catch (error) {
      console.error("Error fetching data: ", error);
    }
  }

  return <div className="mt-6 grid grid-cols-3 gap-2">
    <div className="grid grid-cols-2 gap-2 mx-2">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" className="h-16 text-base">Select topic</Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuRadioGroup value={`(${topic[1]}) ${topic[0]}`} onValueChange={(newValue) => handleTopicChange(newValue)}>
            <DropdownMenuRadioItem value="(0) Any">(0) Any</DropdownMenuRadioItem>
            {topics.map((topic) => (
              <DropdownMenuRadioItem key={`${topic.id}`} value={`(${topic.id}) ${topic.topic}`}>
                {`(${topic.id}) ${topic.topic}`}
              </DropdownMenuRadioItem>
            ))}
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      <DropdownMenu>
        <DropdownMenuTrigger asChild disabled={topic[1] === 0}>
          <Button variant="outline" className="h-16 text-base">Select type</Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuRadioGroup value={`(${topic[1]} ${topic[0]})`} onValueChange={(newValue) => handleQuestionTypeChange(newValue)}>
            <DropdownMenuRadioItem value="(0) Any">(0) Any</DropdownMenuRadioItem>
            {topics.map((currentTopic) => currentTopic.id === topic[1] ?
              currentTopic.question_types && currentTopic.question_types.map((qtype) => (
                <DropdownMenuRadioItem key={`${currentTopic.id}-${qtype.id}`} value={`(${qtype.id}) ${qtype.qtype}`}>
                  {`(${qtype.id}) ${qtype.qtype}`}
                </DropdownMenuRadioItem>
              ))
              : "")}
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>
      <Button onClick={() => handleGenerate()} variant="secondary" className="col-span-2 h-32 mt-1 text-xl">Generate</Button>
    </div>
    <Card className="col-span-2 text-left">
      <CardHeader className="grid grid-cols-2">
        <div>
          <CardDescription className="text-base">
            <span className="font-semibold">Topic:&nbsp;</span>{topic[0]}
          </CardDescription>
          <CardDescription className="text-base">
            <span className="font-semibold">Problem type:&nbsp;</span>{questionType[0]}
          </CardDescription>
        </div>
        <div className="flex justify-end">
          {answerRevealed ?
            <Button onClick={() => setAnswerRevealed(false)} disabled={!questionGenerated} variant="secondary">Hide Answer</Button>
            :
            <Button onClick={() => setAnswerRevealed(true)} disabled={!questionGenerated}>Reveal Answer</Button>
          }
        </div>
      </CardHeader>
      <CardContent>
        {questionGenerated ?
          <div>
            <p className="mb-2">
              <span className="font-extrabold">Question:&nbsp;</span>{question}
            </p>
            {answerRevealed ? <p>
              <span className="font-extrabold">Answer:&nbsp;</span>{answer}
            </p>
              : ""}
          </div>
          : <p>
            You have not generated anything.
            <br />
            Click &quot;Generate&quot; to start fetching practice problems!
          </p>}
      </CardContent>
    </Card>
  </div >;

}
