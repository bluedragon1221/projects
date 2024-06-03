local lf = love.physics
local Object = require 'classic'

local function make_object(world, x, y, w, h, mass)
    local body = lf.newBody(world, x, y)
    local shape = lf.newRectangleShape(w, h)
    local fixture = lf.newFixture(body, shape, mass)

    return {
        body = body,
        shape = shape,
        fixture = fixture
    }
end

function love.load()
    screen = {
        w = 960,
        h = 540
    }

    love.physics.setMeter(64)
    world = lf.newWorld(0, 9.81*64, true)

    objects = {}
    objects.ground = make_object(world, 0, screen.h - 10, screen.w, 10)
    objects.character = make_object(world, 0, 0, 60, 60, 1)
    objects.character.fixture:setRestitution(0.9)

    love.window.setMode(screen.w, screen.h)
end

function love.update(dt)
    world:update(dt)

    if love.keyboard.isDown('left') then
        objects.character.body.applyForce(400, 0)
    elseif love.keyboard.isDown('right') then
        objects.character.body.applyForce(-400, 0)
    elseif love.keyboard.isDown('up') or love.keyboard.isDown('space') then
        objects.character.body:setPosition()
    end
end

