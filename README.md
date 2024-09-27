# Solana Blockchain School

This project implements a decentralized school management system on the Solana blockchain. It allows for student enrollment, class registration, and book distribution, all managed through blockchain technology and NFTs.

## Features

- School account management with administrator controls
- Student enrollment with unique NFT-based student IDs
- Class registration system
- Digital book distribution through NFTs
- Enrollment fee management

## Key Components

### School Account
- `authority`: Public key of the school administrator
- `enrollmentFee`: Cost to enroll in the school (in SOL or lamports)
- `classRegistry`: Mapping of class names to their NFT metadata
- `bookRegistry`: Mapping of book names to their NFT metadata

### Student Account
- `studentId`: Unique identifier for the student
- `studentNft`: Address of the student's ID NFT
- `enrolledClasses`: List of class names the student is enrolled in
- `ownedBooks`: List of book names the student owns

## Program Functions

### `enroll`
Allows a student to enroll in the school.
- Verifies payment of enrollment fee
- Mints a new student ID NFT
- Stores student information in the School account
- Transfers NFT ownership to the student

### `registerClass`
Enables a student to register for a class.
- Verifies student enrollment
- Checks class existence in `classRegistry`
- Updates student's `enrolledClasses` list
- Optionally mints a class NFT for the student

### `requestBook`
Allows a student to request a book.
- Verifies student enrollment
- Checks book existence in `bookRegistry`
- Verifies student meets requirements
- Mints and transfers book NFT to the student

### `updateClassRegistry`
Allows the school administrator to update the class registry.
- Restricted to school administrator
- Adds or updates class information in `classRegistry`

### `updateBookRegistry`
Allows the school administrator to update the book registry.
- Restricted to school administrator
- Adds or updates book information in `bookRegistry`

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

2. Install dependencies:
   ```bash
   anchor build
   ```

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
