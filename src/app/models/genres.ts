import { Book } from "./books";

interface Genre {
  name: string;
  books_amount: number;
}

interface GenreDetails {
  name: string;
  books: Book[];
  books_amount: number;
}

export { Genre, GenreDetails };
