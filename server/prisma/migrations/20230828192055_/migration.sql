-- CreateTable
CREATE TABLE "Animation" (
    "index" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "display_name" TEXT NOT NULL,
    "default_speed" INTEGER NOT NULL
);

-- CreateTable
CREATE TABLE "Color" (
    "index" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "default" TEXT NOT NULL,
    "animationIndex" INTEGER,
    CONSTRAINT "Color_animationIndex_fkey" FOREIGN KEY ("animationIndex") REFERENCES "Animation" ("index") ON DELETE SET NULL ON UPDATE CASCADE
);
