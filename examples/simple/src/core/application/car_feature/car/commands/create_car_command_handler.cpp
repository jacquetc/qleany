// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_car_command_handler.h"
#include "car/validators/create_car_command_validator.h"
#include "tools/automapper.h"

using namespace Simple;
using namespace Simple::Entities;
using namespace Simple::Contracts::DTO::Car;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Car::Validators;
using namespace Simple::Application::Features::Car::Commands;

CreateCarCommandHandler::CreateCarCommandHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<CarDTO> CreateCarCommandHandler::handle(QPromise<Result<void>> &progressPromise, const CreateCarCommand &request)
{
    Result<CarDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateCarCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<CarDTO> CreateCarCommandHandler::restore()
{
    Result<CarDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateCarCommand restore:" << ex.what();
    }
    return result;
}

Result<CarDTO> CreateCarCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const CreateCarCommand &request)
{
    qDebug() << "CreateCarCommandHandler::handleImpl called";
    Simple::Entities::Car car;
    CreateCarDTO createDTO = request.req;

    if (m_firstPass)
    {
        // Validate the create Car command using the validator
        auto validator = CreateCarCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(CarDTO, validatorResult);

        // Map the create Car command to a domain Car object and
        // generate a UUID
        car = Simple::Tools::AutoMapper::map<CreateCarDTO, Simple::Entities::Car>(createDTO);

        // allow for forcing the uuid
        if (car.uuid().isNull())
        {
            car.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        car.setCreationDate(QDateTime::currentDateTime());
        car.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        car = m_newEntity.value();
    }

    // Add the car to the repository

    m_repository->beginChanges();
    auto carResult = m_repository->add(std::move(car));

    QLN_RETURN_IF_ERROR_WITH_ACTION(CarDTO, carResult, m_repository->cancelChanges();)

    // Get the newly created Car entity
    car = carResult.value();
    // Save the newly created entity
    m_newEntity = carResult;

    //  Manage relation to owner

    m_repository->saveChanges();

    m_newEntity = carResult;

    auto carDTO = Simple::Tools::AutoMapper::map<Simple::Entities::Car, CarDTO>(carResult.value());
    Q_EMIT carCreated(carDTO);

    qDebug() << "Car added:" << carDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Car as a Result object
    return Result<CarDTO>(carDTO);
}

Result<CarDTO> CreateCarCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(QList<int>() << entityId);

    QLN_RETURN_IF_ERROR(CarDTO, deleteResult)

    Q_EMIT carRemoved(deleteResult.value().value(Simple::Entities::Entities::EntityEnum::Car).first());

    qDebug() << "Car removed:" << deleteResult.value();

    return Result<CarDTO>(CarDTO());
}

bool CreateCarCommandHandler::s_mappingRegistered = false;

void CreateCarCommandHandler::registerMappings()
{
    Simple::Tools::AutoMapper::registerMapping<Simple::Entities::Car, Contracts::DTO::Car::CarDTO>(true, true);
    Simple::Tools::AutoMapper::registerMapping<Contracts::DTO::Car::CreateCarDTO, Simple::Entities::Car>();
}