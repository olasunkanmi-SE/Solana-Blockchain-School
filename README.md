# Solana Blockchain School

This project implements a decentralized school management system on the Solana blockchain using Program Derived Addresses (PDAs). It allows for student enrollment, class registration, and book distribution, all managed through blockchain technology and NFTs.

## Features

- School account management with administrator controls
- Student enrollment with unique NFT-based student IDs
- Class registration system using PDAs
- Digital book distribution through NFTs and PDAs
- Enrollment fee management

## Key Components

### School Account

- `authority`: Public key of the school administrator
- `enrollmentFee`: Cost to enroll in the school (in SOL or lamports)
- `classCount`: Number of registered classes
- `bookCount`: Number of registered books
- `studentCount`: Number of Students registered

### Course Account (PDA)

- `name`: Name of the Course
- `nftMetadata`: Address of the class NFT metadata
- `capacity`: Maximum number of students
- `enrolledStudentsCount`: Current number of enrolled students
- `tuitionFee`: Cost to attend a course

### Book Account (PDA)

- `name`: Name of the book
- `nftMetadata`: Address of the book NFT metadata
- `totalSupply`: Total number of copies
- `availableCopies`: Number of available copies

### Student Account

- `studentId`: Unique identifier for the student
- `studentNft`: Address of the student's ID NFT
- `enrolledClasses`: List of class PDAs the student is enrolled in
- `ownedBooks`: List of book PDAs the student owns

## Program Functions

### `initializeSchool`

Initializes the main School account.

### `addClass`

Adds a new class to the school, creating a new PDA for the class.

### `addBook`

Adds a new book to the school, creating a new PDA for the book.

### `enroll`

Allows a student to enroll in the school.

### `registerForClass`

Enables a student to register for a class using the class PDA.

### `requestBook`

Allows a student to request a book using the book PDA.

### `getClass`

Retrieves information about a specific class using its PDA.

### `getBook`

Retrieves information about a specific book using its PDA.

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor framework

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/solana-blockchain-school.git
   cd solana-blockchain-school
   ```

````

2. Install dependencies:

   ```bash
   anchor build
````

3. Update the program ID in `lib.rs` and `Anchor.toml` with your program ID:
   ```bash
   solana address -k target/deploy/blockchain_school-keypair.json
   ```

## Usage

1. Implement the functions described above in `lib.rs`.
2. Build the program:
   ```bash
   anchor build
   ```
3. Deploy the program to your desired Solana cluster:
   ```bash
   anchor deploy
   ```
4. Interact with the program using a client application or Anchor tests.

## Testing

To run the tests:

```bash
anchor test
```

## Future Enhancements

- Implement a grading system
- Add functionality for issuing blockchain-based certificates or diplomas
- Develop a front-end application for easy interaction with the school system

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

## Disclaimer

This is an experimental project. Ensure compliance with educational regulations and data privacy laws when implementing in a real-world scenario.
