interface Book {
    id: number,
    title: string,
    cover_path: string,
    author_id: number,
    author_name: string,
}

interface BookDetails extends Book {
    duration: number,
    year: number,
    genre_id: number,
    genre_name: string,
    author_picture_path: string,
    lector_id: number,
    lector_name: string,
}


export { Book, BookDetails };

