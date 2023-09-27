import re
from enum import Enum

from pydantic import BaseModel

RE = r"""(?x)
        \[
            (?P<year>[0-9]+)-(?P<month>[0-9]+)-(?P<day>[0-9]+)
            \s+
            (?P<minute>[0-9]+):(?P<second>[0-9]+)
        \] 
        \s+
        (?:Guard\ \#(?P<id>[0-9]+)|(?P<sleep>.+))
    """


class DateTime(BaseModel):
    year: int
    month: int
    day: int
    minute: int
    second: int


class EventKind(Enum):
    START = 0
    SLEEPS = 0
    WAKES = 0


class Event(BaseModel):
    datetime: DateTime
    kind: EventKind


def parse(line: str) -> Event:
    p = re.compile(RE)
    m = p.match(line)

    if m:
        datetime = DateTime(
            year=int(m.group("year")),
            month=int(m.group("month")),
            day=int(m.group("day")),
            minute=int(m.group("minute")),
            second=int(m.group("second")),
        )
        if m.group("id"):
            kind = EventKind.START
        elif m.group("sleep") == "falls asleep":
            kind = EventKind.SLEEPS
        elif m.group("sleep") == "wakes up":
            kind = EventKind.WAKES
        else:
            raise Exception("Could not determine the event kind")

        return Event(datetime=datetime, kind=kind)
    raise Exception("Could not parse the line")


if __name__ == "__main__":
    file = "input/input.txt"

    with open(file) as f:
        for line in f:
            event = parse(line)
            """ print(event) """
