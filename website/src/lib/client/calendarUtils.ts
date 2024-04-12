export class CalendarDate {
    month: number;
    day: number;
    year: number;
    constructor(dateOrMonth: Date | number, day?: number, year?: number) {
        if (dateOrMonth instanceof Date) {
            this.month = dateOrMonth.getMonth();
            this.day = dateOrMonth.getDate();
            this.year = dateOrMonth.getFullYear();
        } else {
            this.month = dateOrMonth;
            this.day = day!;
            this.year = year!;
        }
    }

    toString(): string {
        return `${this.year}-${(this.month+1).toString().padStart(2, '0')}-${this.day.toString().padStart(2, '0')}`
    }

    getDate(): Date {
        return new Date(Date.UTC(this.year, this.month, this.day))
    }

    nextMonth(months: number = 1): CalendarDate {
        if (months === 0) return this;
        if (this.month === 11) this.year += 1;
        this.month = modulus(this.month + 1, 12);
        return this.nextMonth(--months);
    }

    prevMonth(months: number = 1): CalendarDate {
        if (months === 0) return this;
        if (this.month === 0) this.year -= 1;
        this.month = modulus(this.month - 1, 12);
        return this.prevMonth(--months);
    }

    totalDays(): number {
        let days = 0;
        const leapyears = Math.floor((this.year - 1 - 1972) / 4);
        days += leapyears * 366;
        days += (this.year - 1972 - leapyears) * 365;
        days += totalDaysInMonth(this.year)[this.month] + this.day;
        return days;
    }

    beginningCalendarOffset(): number {
        return firstDayInMonth(this.month + 1, 1, this.year);
    }

    endCalendarOffset(): number {
        const extraSquares = this.numDaysInMonth() + this.beginningCalendarOffset() > 34 ? 0 : 7;
        return (
            7 -
            ((this.beginningCalendarOffset() + numDaysInMonth(this.year)[this.month]) % 7) +
            extraSquares
        );
    }

    numDaysInMonth(): number {
        return numDaysInMonth(this.year)[modulus(this.month, 12)];
    }

    handleClick(day: number): [CalendarDate, string] {
        const clickedDate = new Date(this.year, this.month, day + 1);
        const newFocused = new CalendarDate(clickedDate);
        const inputDateString = `${this.year}-${(this.month + 1).toString().padStart(2, '0')}-${(
            day + 1
        )
            .toString()
            .padStart(2, '0')}`;
        return [newFocused, inputDateString];
    }
}

export const daysOfWeek = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
export const monthNames = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December'
];
export const numDaysInMonth = (year: number): number[] => [31, (year % 400 === 0 || year % 100 !== 0 && year % 4 === 0) ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
export const totalDaysInMonth = (year: number): number[] => {
    if (year % 400 === 0 || (year % 100 !== 0 && year % 4 === 0)) {
        return [
            0,
            31,
            60,
            91,
            121,
            152,
            182,
            213,
            244,
            274,
            305,
            335,
        ]
    }
    return [
        0,
        31,
        59,
        90,
        120,
        151,
        181,
        212,
        243,
        273,
        304,
        334,
    ]
};
export function modulus(dividend: number, divisor: number) {
    let remainder;

    if (dividend < 0) {
        remainder = (dividend + divisor) % divisor
        return remainder
    }

    remainder = dividend % divisor
    return remainder;
}
/**
 * Zeller's Congruence formula to get the first day of month
 * @param {number} month (0-11)
 * @param {number} day (1-31)
 * @param {number} year (1970 - present)
*/
function firstDayInMonth(month: number, day: number, year: number) {
    if (month < 3) {
        month += 12;
        year -= 1;
    }

    const K = year % 100;
    const J = Math.floor(year / 100);

    const dayOfWeek =
        (day +
            Math.floor((13 * (month + 1)) / 5) +
            K +
            Math.floor(K / 4) +
            Math.floor(J / 4) -
            2 * J) %
        7;
    // + 6 offsets the result to match our daysOfWeek array
    return (dayOfWeek + 6) % 7;
}

export {
    firstDayInMonth
}