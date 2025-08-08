-- pipelines
[1, 2, 3]
    |> map(it) {x} x + 1
    |> each(it) {x} println(x)

pub mod vec2[T] = struct { x : T, y : T } where

    pub fn new(x , y) = struct { x, y }

    pub fn add(self, other) = struct {
        x = self.x + other.x,
        y = self.y + other.y,
    }

end

local v = vec2[int]::new(1, 3)
local u = v.add(vec2[int]::new(4, 5))

pub mod diagnostic = ... where
    ...
end

diagnostic::fatal()
    .message("hi")
    .label(location)
    .report(self.issues)

-- same as
diagnostic::fatal()
    |> it.message("hi")
    |> it.label(location)
    |> it.report(self.issues)

-- same as
diagnostic::fatal()
    |> diagnostic::message(it, "hi")
    |> diagnostic::label(it, location)
    |> diagnostic::report(it, self.issues)

-- same as
diagnostic::report(
    diagnostic::label(
        diagnostic::message(
            diagnostic::fatal(), "hi"), location), self.issues)