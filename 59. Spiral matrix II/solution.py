from enum import Enum
class Dir(Enum):
    Up = 0
    Down = 1
    Left = 2
    Right = 3
class Solution:
    def generateMatrix(self, n: int) -> List[List[int]]:
        res = [[0] * n for _ in range(n)]
        up,down,left,right = 0,n,-1,n
        direction = Dir.Right
        i = 0
        j = 0
        x = 1
        while x <= n**2:
            match direction:
                case Dir.Right:
                    while j < right:
                        res[i][j] = x
                        j += 1
                        x += 1
                    i,j = i+1,j-1
                    direction = Dir.Down
                    right -= 1
                case Dir.Down:
                    while i < down:
                        res[i][j] = x
                        i += 1
                        x += 1
                    i,j = i-1,j-1
                    direction = Dir.Left
                    down -= 1
                case Dir.Left:
                    while j > left:
                        res[i][j] = x
                        j -= 1
                        x += 1
                    i,j = i-1,j+1
                    direction = Dir.Up
                    left += 1
                case Dir.Up:
                    while i > up:
                        res[i][j] = x
                        i -= 1
                        x += 1
                    i,j = i+1,j+1
                    up += 1
                    direction = Dir.Right
        return res                       



