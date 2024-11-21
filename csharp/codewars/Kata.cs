using System.Linq;

namespace codewars;

public class Kata
{
    public static double[] Tribonacci(double[] signature, int n)
    {
        if (n < 3) return signature.Take(n).ToArray();

        var result = signature;

        while (result.Length < n) result = result.Append(Sum(result)).ToArray();

        return result;

        double Sum(IEnumerable<double> cur) => cur.Reverse().Take(3).Sum();
    }

    public static int CountBits(int n)
    {
        var step = n;
        var count = 0;
        while (step > 0)
        {
            if (step % 2 == 1) count++;
            step >>= 1;
        }

        return count;
    }

    public static bool IsValidWalk(string[] walk)
    {
        if (walk.Length != 10) return false;

        (int North, int East) location = (0, 0);

        foreach (var direction in walk)
            switch (direction)
            {
                case "n":
                    location.North += 1;
                    break;
                case "s":
                    location.North -= 1;
                    break;
                case "w":
                    location.East -= 1;
                    break;
                case "e":
                    location.East += 1;
                    break;
            }

        return location is { North: 0, East: 0 };
    }
}