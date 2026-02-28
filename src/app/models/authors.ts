import { Book } from "./books";

interface Author {
  name: string;
  relative_img_path: string;
}

interface AuthorListItem {
  name: string;
  relative_img_path: string;
  books_count: number;
}

interface AuthorDetails {
  author: Author;
  books: Book[];
}

export { Author, AuthorListItem, AuthorDetails };
