namespace WasmDayDotnet;

public static class App
{
    [ConsoleHandler]
    public static InteropString HandleConsoleInput(InteropString input)
    {
        RandomThingRequest randomThingRequest = input.ToString().Trim() switch
        {
            "joke" => JokeRequest(),
            "cat" => AnimalFactRequest(RandomThingAnimalType.RANDOM_THING_ANIMAL_TYPE_CAT),
            "dog" => AnimalFactRequest(RandomThingAnimalType.RANDOM_THING_ANIMAL_TYPE_DOG),
            // The error handling wouldn't be informative so just do cats BECAUSE CATS
            _ => AnimalFactRequest(RandomThingAnimalType.RANDOM_THING_ANIMAL_TYPE_CAT),
        };

        var randomThingResponse = default(InteropStringOrRandomThingError);

        RandomThingInterop.random_thing_get_random_thing(ref randomThingRequest, ref randomThingResponse);

        string responseText;
        if (randomThingResponse.is_err == 0)
        {
            responseText = randomThingResponse.val.ok.ToString();
        }
        else
        {
            var err = randomThingResponse.val.err;
            var errorText = err.tag switch
            {
                RandomThingError.RANDOM_THING_ERROR_NETWORK => err.val.network.ToString(),
                RandomThingError.RANDOM_THING_ERROR_SERVICE => $"Service status code {err.val.service}",
                RandomThingError.RANDOM_THING_ERROR_RESPONSE => "Response was not text",
                _ => "Inconceivable!"
            };
            responseText = $"Error: {errorText}";
        }

        return InteropString.FromString(responseText);
    }

    private static RandomThingRequest JokeRequest()
    {
        return new RandomThingRequest
        {
            tag = RandomThingRequest.RANDOM_THING_REQUEST_JOKE,
            val = default(RandomThingRequestVal),
        };
    }

    private static RandomThingRequest AnimalFactRequest(byte animalType)
    {
        return new RandomThingRequest
        {
            tag = RandomThingRequest.RANDOM_THING_REQUEST_ANIMAL_FACT,
            val = new RandomThingRequestVal
            {
                animalFact = new RandomThingAnimalType
                {
                    animalType = animalType,
                },
            },
        };
    }
}
