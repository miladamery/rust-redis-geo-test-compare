package ir.desprime.TestGeoSpatialReactive;

import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.geo.*;
import org.springframework.data.redis.connection.RedisGeoCommands;
import org.springframework.data.redis.core.GeoOperations;
import org.springframework.data.redis.core.ReactiveStringRedisTemplate;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.util.List;
import java.util.stream.Collectors;

@Service
@RequiredArgsConstructor
public class GeoService {

    public static final String VENUS_VISITED = "venues_visited";

    private final ReactiveStringRedisTemplate redisTemplate;

    public void add(Location location) {
        Point point = new Point(location.getLng(), location.getLat());
        redisTemplate.opsForGeo().add(VENUS_VISITED, point, location.getName()).block();
    }

    public Mono<List<String>> nearByVenues(Double lng, Double lat, Double kmDistance) {
        Circle circle = new Circle(new Point(lng, lat), new Distance(kmDistance, Metrics.KILOMETERS));
        return redisTemplate
                .opsForGeo()
                .radius(VENUS_VISITED, circle)
                .map(GeoResult::getContent)
                .map(RedisGeoCommands.GeoLocation::getName)
                .collectList()
        ;
    }

}
