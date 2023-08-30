// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_brand_query_handler.h"
#include "repository/interface_brand_repository.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Application::Features::Brand::Queries;

GetAllBrandQueryHandler::GetAllBrandQueryHandler(InterfaceBrandRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<BrandDTO>> GetAllBrandQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllBrandQueryHandler::handle called";

    Result<QList<BrandDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<BrandDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllBrandQuery:" << ex.what();
    }
    return result;
}

Result<QList<BrandDTO>> GetAllBrandQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllBrandQueryHandler::handleImpl called";

    // do
    auto brandResult = m_repository->getAll();

    if (Q_UNLIKELY(brandResult.isError()))
    {
        return Result<QList<BrandDTO>>(brandResult.error());
    }

    // map
    QList<BrandDTO> dtoList;

    for (const Simple::Domain::Brand &brand : brandResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(brand);
        dtoList.append(dto);
    }

    qDebug() << "GetAllBrandQueryHandler::handleImpl done";

    return Result<QList<BrandDTO>>(dtoList);
}

bool GetAllBrandQueryHandler::s_mappingRegistered = false;

void GetAllBrandQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Brand, Contracts::DTO::Brand::BrandDTO>(
        true);
}