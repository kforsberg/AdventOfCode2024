var fileText = File.ReadAllText("inputs/input.txt");
var split = fileText.Split("\r\n\r\n");
var orderingRules = BuildOrderingRules(split[0]);
var updates = BuildUpdates(split[1]);
var validUpdates = updates.Where(x => IsUpdateValid(x, orderingRules));
var sum = validUpdates.Sum(x => GetMiddleUpdate(x));

Console.WriteLine($"There are {validUpdates.Count()} invalid updates");


IEnumerable<PageOrderRule> BuildOrderingRules(string orderingText)
{
    var orderingRules = new List<PageOrderRule>();
    foreach (var orderRule in orderingText.Split("\n"))
    {
        orderingRules.Add(new PageOrderRule
        {
            First = int.Parse(orderRule.Split("|")[0]),
            Last = int.Parse(orderRule.Split("|")[1]),
        });
    }
    return orderingRules;
}

IEnumerable<IEnumerable<int>> BuildUpdates(string updatesText)
{
    var updates = new List<IEnumerable<int>>();
    foreach (var updateText in updatesText.Split("\r\n"))
    {
        var update = new List<int>();
        foreach (var updateRule in updateText.Split(','))
        {
            update.Add(int.Parse(updateRule));
        }
        updates.Add(update);
    }
    return updates;
}

bool IsUpdateValid(IEnumerable<int> updates, IEnumerable<PageOrderRule> rules)
{
    return !rules.Any(rule => updates.Contains(rule.First) && updates.Contains(rule.Last)
        && updates.ToList().IndexOf(rule.First) >= updates.ToList().IndexOf(rule.Last));
}

int GetMiddleUpdate(IEnumerable<int> updates)
{
    return updates.ToList()[(int)Math.Floor(updates.Count() / 2.0)];
}

class PageOrderRule
{
    public required int First { get; set; }
    public required int Last { get; set; }
}