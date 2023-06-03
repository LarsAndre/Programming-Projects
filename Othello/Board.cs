using System;

namespace Othello
{
    class Board
    {
        // The game board is an 8x8 grid
        private int[,] grid = new int[8, 8];

        public Board()
        {
            // Initialize the game board with the initial piece positions
            for (int i = 0; i < 8; i++)
            {
                for (int j = 0; j < 8; j++)
                {
                    grid[i, j] = 0;
                }
            }

            // The middle four squares should be filled with two pieces from each player
            grid[3, 3] = 1;
            grid[3, 4] = 2;
            grid[4, 3] = 2;
            grid[4, 4] = 1;
        }

        public int[] CountPieces()
        {
            int[] count = new int[3]; // Indices 1 and 2 will hold the counts for player 1 and 2

            for (int i = 0; i < 8; i++)
            {
                for (int j = 0; j < 8; j++)
                {
                    count[grid[i, j]]++;
                }
            }

            return count;
        }

        public void Display()
        {
            // Print column headers
            Console.Write("  ");
            for (int i = 0; i < 8; i++)
            {
                Console.Write((char)('a' + i) + " ");
            }
            Console.WriteLine();

            // Print rows with headers
            for (int i = 0; i < 8; i++)
            {
                Console.Write((i + 1) + " ");
                for (int j = 0; j < 8; j++)
                {
                    Console.Write(grid[i, j] + " ");
                }
                Console.WriteLine();
            }
        }

        public bool IsValidMove(int row, int col, Player player)
        {
            // Check if the cell is empty
            if (grid[row, col] != 0)
            {
                return false;
            }

            // Check all directions: horizontal, vertical, and diagonal
            int[] directions = {-1, 0, 1};
            bool validMove = false;

            foreach (int dx in directions)
            {
                foreach (int dy in directions)
                {
                    // Skip the case where dx and dy are both 0
                    if (dx == 0 && dy == 0)
                    {
                        continue;
                    }

                    // Check if there are opponent's pieces in this direction
                    int x = row + dx;
                    int y = col + dy;
                    bool hasOpponentPieces = false;

                    while (x >= 0 && x < 8 && y >= 0 && y < 8 && grid[x, y] == 3 - player.Number)
                    {
                        x += dx;
                        y += dy;
                        hasOpponentPieces = true;
                    }

                    // Check if there's a player's piece at the end
                    if (x >= 0 && x < 8 && y >= 0 && y < 8 && grid[x, y] == player.Number && hasOpponentPieces)
                    {
                        validMove = true;
                    }
                }
            }

            return validMove;
        }


        public bool HasValidMove(Player player)
        {
            for (int i = 0; i < 8; i++)
            {
                for (int j = 0; j < 8; j++)
                {
                    if (grid[i, j] == 0 && IsValidMove(i, j, player))
                    {
                        return true;
                    }
                }
            }
            return false;
        }


        public bool MakeMove(int row, int col, Player player)
        {
            // Check if the cell is empty
            if (grid[row, col] != 0)
            {
                return false;
            }

            // Check all directions: horizontal, vertical, and diagonal
            int[] directions = {-1, 0, 1};
            bool validMove = false;

            foreach (int dx in directions)
            {
                foreach (int dy in directions)
                {
                    // Skip the case where dx and dy are both 0
                    if (dx == 0 && dy == 0)
                    {
                        continue;
                    }

                    // Check if there are opponent's pieces in this direction
                    int x = row + dx;
                    int y = col + dy;
                    bool hasOpponentPieces = false;

                    while (x >= 0 && x < 8 && y >= 0 && y < 8 && grid[x, y] == 3 - player.Number)
                    {
                        x += dx;
                        y += dy;
                        hasOpponentPieces = true;
                    }

                    // Check if there's a player's piece at the end
                    if (x >= 0 && x < 8 && y >= 0 && y < 8 && grid[x, y] == player.Number && hasOpponentPieces)
                    {
                        // This is a valid move, so flip the opponent's pieces
                        x -= dx;
                        y -= dy;

                        while (x != row || y != col)
                        {
                            grid[x, y] = player.Number;
                            x -= dx;
                            y -= dy;
                        }

                        validMove = true;
                    }
                }
            }

            // If this is a valid move, place the player's piece on the board
            if (validMove)
            {
                grid[row, col] = player.Number;
            }

            return validMove;
        }
    }
}
