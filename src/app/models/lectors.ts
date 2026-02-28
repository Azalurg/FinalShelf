import { Book } from "./books";

interface Lector {
  name: string;
  books_amount: number;
}

interface LectorDetails {
  name: string;
  books: Book[];
  books_amount: number;
}

export { Lector, LectorDetails };
