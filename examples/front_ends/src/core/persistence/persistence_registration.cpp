// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "persistence_registration.h"
#include <qleany/database/database_context.h>
#include <qleany/database/database_table_group.h>

#include "repository/brand_repository.h"
#include "repository/car_repository.h"
#include "repository/client_repository.h"
#include "repository/passenger_repository.h"

using namespace Qleany;
using namespace Qleany::Database;
using namespace Qleany::Repository;
using namespace FrontEnds::Persistence;
using namespace FrontEnds::Persistence::Repository;

PersistenceRegistration::PersistenceRegistration(QObject *parent) : QObject{parent}
{
    QSharedPointer<DatabaseContext> context(new DatabaseContext());

    // database tables:

    auto *passengerDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Passenger>(context);
    auto *brandDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Brand>(context);
    auto *clientDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Client>(context);
    auto *carDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Car>(context);

    Result<void> initResult = context->init();

    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + "\n" + error.code() + "\n" + error.message() + "\n" + error.data();
    }

    // repositories:

    PassengerRepository *passengerRepository = new PassengerRepository(passengerDatabaseTableGroup);
    BrandRepository *brandRepository = new BrandRepository(brandDatabaseTableGroup);
    ClientRepository *clientRepository = new ClientRepository(clientDatabaseTableGroup, passengerRepository);
    CarRepository *carRepository = new CarRepository(carDatabaseTableGroup, brandRepository, passengerRepository);

    // register repositories:

    RepositoryProvider::instance()->registerRepository("passenger", passengerRepository);
    RepositoryProvider::instance()->registerRepository("brand", brandRepository);
    RepositoryProvider::instance()->registerRepository("client", clientRepository);
    RepositoryProvider::instance()->registerRepository("car", carRepository);
}

RepositoryProvider *PersistenceRegistration::repositoryProvider()
{
    return RepositoryProvider::instance();
}