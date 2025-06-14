interface Genre {
  name: string;
  books_amount: number;
}

interface GenreDetails {
  name: string;
  books: string[];
  books_amount: number;
}

export { Genre, GenreDetails };
