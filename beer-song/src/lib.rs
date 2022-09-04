use std::fmt::{Display, Formatter};
// Overkill to practice
pub fn verse(n: u32) -> String {
    Verse(n).into()
}

pub fn sing(start: u32, end: u32) -> String {
    Song(start, end).into()
}

struct Song(u32, u32);
impl From<Song> for String {
    fn from(song: Song) -> Self {
        format!("{}", song)
    }
}

struct Verse(u32);
impl From<Verse> for String {
    fn from(verse: Verse) -> Self {
        format!("{}", verse)
    }
}

#[derive(Copy, Clone)]
enum Style {
    Capitalized,
    Uncapitalized,
}

struct Beers(u32, Style);

struct OnTheWall(Beers);

impl Beers {
    fn new(n: u32) -> Self {
        Self(n, Style::Uncapitalized)
    }
    fn capitalized(&self) -> Self {
        Self(self.0, Style::Capitalized)
    }
    fn on_the_wall(self) -> OnTheWall {
        OnTheWall(self)
    }
    fn pronoun(&self) -> &'static str {
        match self.0 {
            1 => "it",
            _ => "one",
        }
    }
    fn next(&self) -> Self {
        if self.0 == 0 {
            Self(99, self.1)
        } else {
            Self(self.0 - 1, self.1)
        }
    }
    fn count(&self) -> &dyn Display {
        match self {
            Beers(0, Style::Uncapitalized) => &"no more",
            Beers(0, Style::Capitalized) => &"No more",
            Beers(n, _) => n,
        }
    }
    fn bottles(&self) -> &'static str {
        match self.0 {
            1 => "bottle",
            _ => "bottles",
        }
    }
}

impl Display for Beers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} of beer", self.count(), self.bottles())
    }
}

impl Display for OnTheWall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on the wall", self.0)
    }
}

impl Display for Verse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let beers = Beers::new(self.0);
        write!(f, "{}, {}.\n", beers.capitalized().on_the_wall(), beers)?;
        if self.0 == 0 {
            writeln!(
                f,
                "Go to the store and buy some more, {}.",
                beers.next().on_the_wall()
            )
        } else {
            writeln!(
                f,
                "Take {} down and pass it around, {}.",
                beers.pronoun(),
                beers.next().on_the_wall()
            )
        }
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut iter = (self.1..=self.0).rev();
        if let Some(n) = iter.next() {
            write!(f, "{}", Verse(n))?;
        }
        while let Some(n) = iter.next() {
            write!(f, "\n{}", Verse(n))?;
        }
        Ok(())
    }
}
