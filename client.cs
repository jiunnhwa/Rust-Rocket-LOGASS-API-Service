//Client to test insert 100 items.
//output: 20220715 RustLogassClientInsertDemo
//Source: ...\REST_CLIENT_LOGASS.csproj

using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Net.Http.Json;
using System.Text.Json.Nodes;
using System.Diagnostics;

var watch = Stopwatch.StartNew();

foreach (var item in ReadFile())
{
    PostJsonAsync("http://localhost:8000", item);
}

watch.Stop();
Console.WriteLine("\n\nRunTime to insert 100 items. TotalSeconds:" + watch.Elapsed.TotalSeconds );

//Send the json
void PostJsonAsync(string url, string msg)
{
    using (var client = new HttpClient())
    {
        client.BaseAddress = new Uri(url);//base url
        var response = client.PostAsJsonAsync("log", msg).Result;//page
        Console.WriteLine(response);
        if (response.IsSuccessStatusCode)
        {
            Console.WriteLine("Success", response);
        }
        else
            Console.WriteLine("Error", response);
    }
}

//read sample log data
static string[] ReadFile(string path = @"sample_log.txt")
{
    if (File.Exists(path))
    {
        return File.ReadAllLines(path);
    }
    return new string[] { };
}

