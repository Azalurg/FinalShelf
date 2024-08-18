import { BookDetails } from "./books";

interface Author {
  id: number;
  name: string;
  picture_path: string;
}

interface AuthorDetails extends Author {
  books: BookDetails[];
}

export { Author, AuthorDetails };
