package ir.desprime.TestGeoSpatialReactive;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.util.List;

@RestController
public class LocationController {

    private final GeoService geoService;

    public LocationController(GeoService geoService) {
        this.geoService = geoService;
    }

    @GetMapping("/")
    public Mono<List<String>> locations(Double longitude, Double latitude) {
        return geoService.nearByVenues(longitude, latitude, 40.0);
    }

}
