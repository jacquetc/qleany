// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_car_query_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_car_repository.h"

using namespace Qleany;
using namespace Simple::Application::Features::Car::Queries;

GetAllCarQueryHandler::GetAllCarQueryHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<CarDTO>> GetAllCarQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllCarQueryHandler::handle called";

    Result<QList<CarDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<CarDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllCarQuery:" << ex.what();
    }
    return result;
}

Result<QList<CarDTO>> GetAllCarQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllCarQueryHandler::handleImpl called";

    // do
    auto carResult = m_repository->getAll();

    if (Q_UNLIKELY(carResult.isError()))
    {
        return Result<QList<CarDTO>>(carResult.error());
    }

    // map
    QList<CarDTO> dtoList;

    for (const Simple::Domain::Car &car : carResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(car);
        dtoList.append(dto);
    }

    qDebug() << "GetAllCarQueryHandler::handleImpl done";

    return Result<QList<CarDTO>>(dtoList);
}

bool GetAllCarQueryHandler::s_mappingRegistered = false;

void GetAllCarQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Car, Contracts::DTO::Car::CarDTO>(true,
                                                                                                             true);
}