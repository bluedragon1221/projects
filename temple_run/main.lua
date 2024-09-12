map = {
  {1, 1, 1, 1, 1},
  {1, 0, 0, 0, 1},
  {1, 0, 0, 0, 1},
  {1, 0, 0, 0, 0},
  {1, 1, 1, 1, 1}
}

SQUARE_SIZE = 50

function love.draw()
  for index, item in ipairs(map)
    for jndex, jtem in ipairs(i)
      if jtem == 1 then
        love.graphics.rectangle("fill", index*SQUARE_SIZE, jndex*SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE)
      end
    end
  end
end
