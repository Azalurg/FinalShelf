interface Book {
  title: string;
  relative_cover_path: string;
  author_name: string;
  genre: string;
  lector: string;
  create_date: string;
  read: boolean;
  score: number;
  duration_seconds: number | null;
  series_id: number | null;
  series_order: number | null;
}

interface ListResponse<T> {
  items: T[];
  total_count: number;
  page: number;
  limit: number;
  total_pages: number;
}

type BookListResponse = ListResponse<Book>;

export { Book, BookListResponse, ListResponse };
