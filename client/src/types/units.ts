export interface Qtype {
  id: number;
  qtype: string;
}

export interface Topic {
  id: number;
  topic: string;
  question_types: Qtype[]
}
