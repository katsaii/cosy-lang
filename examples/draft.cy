-- pipelines
[1, 2, 3]
    |> map(it) {x} x + 1
    |> each(it) {x} println(x)

pub type Vec2[T] = struct { x : T, y : T } where

    pub fn new(x , y) = struct { x, y }

    pub fn add(self, other) = struct {
        x = self.x + other.x,
        y = self.y + other.y,
    }

end

local v = Vec2[int]::new(1, 3)
local u = v.add(Vec2[int]::new(4, 5))

pub type Diagnostic = ... where
    ...
end

Diagnostic::fatal()
    .message("hi")
    .label(location)
    .report(self.issues)

-- same as
Diagnostic::fatal()
    |> it.message("hi")
    |> it.label(location)
    |> it.report(self.issues)

-- same as
Diagnostic::fatal()
    |> Diagnostic::message(it, "hi")
    |> Diagnostic::label(it, location)
    |> Diagnostic::report(it, self.issues)

-- same as
Diagnostic::report(
    Diagnostic::label(
        Diagnostic::message(
            Diagnostic::fatal(), "hi"), location), self.issues)

do
    local n = #[try] get_res()
    n + 2
end