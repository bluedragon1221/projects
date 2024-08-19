ButtonManager = require("lib.simplebutton")

SQUARE_SIZE = 80

selected_first = nil
selected_second = nil

function love.load()
  love.window.setMode(SQUARE_SIZE*8, SQUARE_SIZE*8)

  color_is_black = true

  for aval = 0, 8*SQUARE_SIZE, SQUARE_SIZE do
    for bval = 0, 8*SQUARE_SIZE, SQUARE_SIZE do
      local color = color_is_black and {0, 0, 0, 1} or {1, 1, 1, 1}
      local pressed_color = color_is_black and {0.2, 0.2, 0.2, 1} or {0.8, 0.8, 0.8, 1}

      local a = ButtonManager.new("", bval, aval, SQUARE_SIZE, SQUARE_SIZE, true, color, pressed_color, pressed_color)

      function a:onToggleOn()
        if selected_first == nil then
          selected_first = { aval/SQUARE_SIZE+1, bval/SQUARE_SIZE+1 }
        elseif selected_first and selected_second == nil then
          selected_second = { aval/SQUARE_SIZE+1, bval/SQUARE_SIZE+1 }
        end
      end

      color_is_black = not color_is_black
    end
  end
end

function love.draw()
  ButtonManager.draw()
  if selected_first then
    print(selected_first[1]..", "..selected_first[2])
  end
  if selected_second then
    print(selected_second[1]..","..selected_second[2])
  end
  print("------")
end

function love.mousepressed(x, y, msbutton, istouch, presses)
   ButtonManager.mousepressed(x,y,msbutton)
end

function love.mousereleased(x, y, msbutton, istouch, presses)
   ButtonManager.mousereleased(x,y,msbutton)
end
