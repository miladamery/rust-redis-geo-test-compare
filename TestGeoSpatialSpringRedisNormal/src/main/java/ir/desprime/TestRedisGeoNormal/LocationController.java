package ir.desprime.TestRedisGeoNormal;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
public class LocationController {

    private final GeoService geoService;

    public LocationController(GeoService geoService) {
        this.geoService = geoService;
    }

    @GetMapping("/")
    public ResponseEntity<List<String>> locations(Double longitude, Double latitude) {
        List<String> locations = geoService.nearByVenues(longitude, latitude, 40.0);
        return ResponseEntity.ok(locations);
    }

}
