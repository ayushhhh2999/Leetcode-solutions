class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        m = len(matrix)
        n = len(matrix[0])
        UP, DOWN, LEFT, RIGHT = 0, 1, 2, 3
        up_w, down_w, left_w, right_w = 0, m, -1, n
        res = []
        Direction = RIGHT
        i = 0
        j = 0
        while len(res) != m * n:
            if Direction == RIGHT:
                while j < right_w:
                    res.append(matrix[i][j])
                    j += 1
                right_w -= 1
                i, j = i + 1, j - 1
                Direction = DOWN
            elif Direction == DOWN:
                while i < down_w:
                    res.append(matrix[i][j])
                    i += 1
                down_w -= 1
                i, j = i - 1, j - 1
                Direction = LEFT
            elif Direction == LEFT:
                while j > left_w:
                    res.append(matrix[i][j])
                    j -= 1
                left_w += 1
                i, j = i - 1, j + 1
                Direction = UP
            else:
                while i > up_w:
                    res.append(matrix[i][j])
                    i -= 1
                up_w += 1
                i, j = i + 1, j + 1
                Direction = RIGHT
        return res