import { Book } from "./books";

interface Author {
  name: string;
  relative_img_path: string;
}

interface AuthorDetails {
  author: Author;
  books: Book[];
}

export { Author, AuthorDetails };
