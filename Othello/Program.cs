using System;

namespace Othello
{
    class Program
    {
        static void Main(string[] args)
        {
            // Create a new game board
            Board board = new Board();

            // Create two players
            Player player1 = new Player("Player 1", 1);
            Player player2 = new Player("Player 2", 2);

            // Put the players in an array
            Player[] players = { player1, player2 };

            // Set the current player index to 0 (player 1)
            int currentPlayerIndex = 0;

            // Main game loop
            while (true)
            {
                // Display the game board
                board.Display();

                // Get the current player
                Player currentPlayer = players[currentPlayerIndex];

                // Check if the current player has any valid moves
                if (!board.HasValidMove(currentPlayer))
                {
                    Console.WriteLine($"Player {currentPlayer.Number} has no valid moves and must pass their turn.");
                    currentPlayerIndex = (currentPlayerIndex + 1) % players.Length;
                    continue;
                }

                // Get the player's move
                Console.Write($"Player {currentPlayer.Number}, enter your move (col row): ");
                string[] input = Console.ReadLine().Split();

                // Check if the input is in the correct format
                if (input.Length != 2 || input[0].Length != 1 || !char.IsLetter(input[0][0]) || !int.TryParse(input[1], out int row))
                {
                    Console.WriteLine("Invalid input format. Please enter your move in the format 'col row', where 'col' is a letter from 'a' to 'h' and 'row' is a number from 1 to 8.");
                    continue;
                }

                row--; // Subtract 1 because array indices start at 0
                int col = input[0][0] - 'a'; // Subtract 'a' to convert from letters to numbers

                // Check if the move is within the board
                if (row < 0 || row >= 8 || col < 0 || col >= 8)
                {
                    Console.WriteLine("Invalid move. The move must be within the board.");
                    continue;
                }

                // Make the move
                bool validMove = board.MakeMove(row, col, currentPlayer);

                // If the move is valid, switch to the next player
                if (validMove)
                {
                    currentPlayerIndex = (currentPlayerIndex + 1) % players.Length;
                }
                else
                {
                    Console.WriteLine("Invalid move. You must place your piece next to an opponent's piece such that it surrounds at least one of the opponent's pieces.");
                }

                // Check if a player has lost all of their pieces
                int[] pieceCount = board.CountPieces();
                if (pieceCount[1] == 0)
                {
                    Console.WriteLine("Player 2 wins! Player 1 has no pieces left.");
                    break;
                }
                else if (pieceCount[2] == 0)
                {
                    Console.WriteLine("Player 1 wins! Player 2 has no pieces left.");
                    break;
                }
            }
        }
    }
}
