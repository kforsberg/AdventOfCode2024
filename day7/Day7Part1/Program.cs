var fileText = File.ReadAllText("inputs/input.txt");
var fileLines = fileText.Split("\r\n");

var operations = new List<Func<(long, long), long>>
{
    (l) => l.Item1 + l.Item2,
    (l) => l.Item1 * l.Item2
};

var equations = new List<Equation>();

foreach (var line in fileLines)
{
    equations.Add(BuildEquation(line));
}

Console.WriteLine(Solve(operations, equations));

Equation BuildEquation(string line)
{
    var split = line.Split(':');
    var splitNum = split[1].Split(' ');
    return new Equation
    {
        TestValue = long.Parse(split[0]),
        Items = split[1].Split(' ').Where(s => !string.IsNullOrWhiteSpace(s)).Select(s => long.Parse(s))
    };
}

long Solve(List<Func<(long, long), long>> operations, IEnumerable<Equation> equations)
{
    long total = 0L;
    foreach (var equation in equations)
    {
        if (HasValidSolution(operations, equation.TestValue, 0L, equation.Items))
        {
            total += equation.TestValue;
        }
    }
    return total;
}

bool HasValidSolution(List<Func<(long, long), long>> operations, long testValue, long sum, IEnumerable<long> remaining)
{
    if (!remaining.Any())
    {
        return testValue == sum;
    }
    else if (sum > testValue)
    {
        return false;
    } 
    else
    {

        return operations.Any(o => HasValidSolution(operations, testValue, o.Invoke((sum, remaining.ToList()[0])), remaining.ToList().Skip(1)));
    }
}

class Equation
{
    public long TestValue { get; set; }
    public IEnumerable<long> Items { get; set; } = [];
}
