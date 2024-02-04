package ir.desprime;

import java.time.Duration;
import java.util.Random;
import static io.gatling.javaapi.core.CoreDsl.*;
import static io.gatling.javaapi.http.HttpDsl.*;

import io.gatling.javaapi.core.*;
import io.gatling.javaapi.http.*;

public class FinderLoadSimulation extends Simulation {

    private HttpProtocolBuilder httpProtocol = http
            .baseUrl("http://localhost:8085");

    private ScenarioBuilder scn = scenario("New Finder Load Test")
            .exec(http("get nearest locations").get(session -> "/?" + generateCoordinates()));

    {
        setUp(
                scn.injectOpen(
                        rampUsers(1).during(Duration.ofSeconds(10)),
                        constantUsersPerSec(100).during(Duration.ofSeconds(10)),
                        //rampUsersPerSec(1000).to(10000).during(Duration.ofSeconds(10)),
                        constantUsersPerSec(1000).during(Duration.ofSeconds(60)),
                        rampUsers(1).during(Duration.ofSeconds(30))
                )
        ).protocols(httpProtocol);
    }

    private static String generateCoordinates() {
        Random random = new Random();

        // France bounds (approximate values)
        double minLat = 41.303;
        double maxLat = 51.124;
        double minLon = -5.725;
        double maxLon = 9.562;

        double latitude = minLat + (maxLat - minLat) * random.nextDouble();
        double longitude = minLon + (maxLon - minLon) * random.nextDouble();
        return "longitude=" + longitude + "&latitude=" + latitude;
    }
}
