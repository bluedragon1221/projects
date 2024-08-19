block = {
  x = 50,
  vx = 0,
  y = 50,
  vy = 10,
  w = 50,
  h = 50,

  is_jumping = false,
}

game = {
  WIDTH = love.graphics.getWidth(),
  HEIGHT = love.graphics.getHeight(),

  JUMP_VY = -20,
  MOVE_VX = 10,
  MOVE_FRICTION = 2.5,
  GRAVITY = 1.5,
}

function block:draw()
  love.graphics.rectangle("fill", self.x, self.y, self.w, self.h)
end

function block:isTouchingWall(wall)
  if wall == "bottom" and self.y > (game.HEIGHT - self.h) then
    return true
  elseif wall == "top" and self.y < 0 then
    return true
  elseif wall == "left" and self.x < 0 then
    return true
  elseif wall == "right" and self.x > (game.WIDTH - self.w) then
    return true
  else
    return false
  end
end

function block:applyGravity()
  self.y = self.y + self.vy

  if (self.x > 0 and self.vx < 0) or
     ((game.WIDTH - self.w) > self.x and self.vx > 0) then
    self.x = self.x + self.vx
  end

  -- make sure block isn't clipping off screen x
  if block:isTouchingWall("left") then
    self.x = 0
  elseif block:isTouchingWall("right") then
    self.x = game.WIDTH - self.w
  end

  if block:isTouchingWall("bottom") then
    -- make sure block isn't clipping off the screen y
    self.y = game.HEIGHT - self.h

    if self.is_jumping then
      self.is_jumping = false
    else -- not jumping
      self.vy = 0
    end
  else
    -- speed up fall if block isn't on ground
    self.vy = self.vy + game.GRAVITY
  end
end

function block:applyFriction()
  -- make the block return to vx = 0
  if self.vx > 0 then
    self.vx = math.max(0, self.vx - game.MOVE_FRICTION)
  elseif self.vx < 0 then
    self.vx = math.min(0, self.vx + game.MOVE_FRICTION)
  end
end

function block:do_jump()
  self.is_jumping = true
  self.vy = game.JUMP_VY
end

function love.update()
  block:applyGravity()

  if love.keyboard.isDown("left") and not love.keyboard.isDown("right") then
    block.vx = -game.MOVE_VX
  elseif love.keyboard.isDown("right") and not love.keyboard.isDown("left") then
    block.vx = game.MOVE_VX
  else
    block:applyFriction()
  end
end

function love.draw()
  block:draw()
end

function love.keypressed(key, scancode, isRepeat)
  if key == "space" and not block.is_jumping then
    block:do_jump()
  end
end
